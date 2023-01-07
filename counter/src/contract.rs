#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128 };
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::increment_counter;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg };
use crate::query::{ get_config, get_counter };
use crate::state::{ Config, CONFIG, COUNTER };

// version info for migration info
const CONTRACT_NAME: &str = "counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    COUNTER.save(deps.storage, &Uint128::new(0))?;

    let admin_address = info.sender;
    let config = Config {
        admin_address,
    };

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::IncrementCounter {} => increment_counter(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => get_config(deps, env),
        QueryMsg::GetCounter {} => get_counter(deps, env),
    }
}