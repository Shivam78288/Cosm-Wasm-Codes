use cosmwasm_std::{ Deps, Env, Binary, StdResult, to_binary };
use crate::state::{ COUNTER, CONFIG };

pub fn get_counter(deps: Deps, _env: Env) -> StdResult<Binary> {
    let counter = COUNTER.may_load(deps.storage)?;
    to_binary(&counter)
}

pub fn get_config(deps: Deps, _env: Env) -> StdResult<Binary> {
    let config = CONFIG.may_load(deps.storage)?;
    to_binary(&config)
}