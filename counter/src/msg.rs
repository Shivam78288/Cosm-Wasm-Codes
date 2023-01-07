use cosmwasm_schema::{ cw_serde, QueryResponses };
use cosmwasm_std::Uint128;
use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    IncrementCounter {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)] GetCounter {},
    #[returns(Config)] GetConfig {},
}