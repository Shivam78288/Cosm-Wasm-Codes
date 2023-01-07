use std::ops::Add;
use cosmwasm_std::{ DepsMut, Env, MessageInfo, Response, Uint128 };
use crate::{ ContractError, state::COUNTER };

pub fn increment_counter(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo
) -> Result<Response, ContractError> {
    let counter = COUNTER.load(deps.storage)?;
    let counter = counter.add(Uint128::new(1));
    COUNTER.save(deps.storage, &counter)?;

    Ok(Response::new().add_attribute("action", "increment_counter"))
}