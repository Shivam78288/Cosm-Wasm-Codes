#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{ mock_dependencies, mock_info, mock_env },
        Attribute,
        from_binary,
        Uint128,
    };

    use crate::{
        msg::{ InstantiateMsg, ExecuteMsg, QueryMsg },
        contract::{ instantiate, execute, query },
        state::Config,
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);
        let env = mock_env();
        let msg = InstantiateMsg {};

        // instantiating
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "instantiate")])
    }

    #[test]
    fn test_increment_counter() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);
        let env = mock_env();

        let msg = InstantiateMsg {};

        // instantiating
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "instantiate")]);

        // increment counter
        let msg = ExecuteMsg::IncrementCounter {};
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "increment_counter")])
    }

    #[test]
    fn get_counter() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);
        let env = mock_env();

        let msg = InstantiateMsg {};

        // instantiating
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "instantiate")]);

        // increment counter
        let msg = ExecuteMsg::IncrementCounter {};
        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "increment_counter")]);

        // get counter
        let msg = QueryMsg::GetCounter {};
        let res = query(deps.as_ref(), env, msg).unwrap();
        let counter: Uint128 = from_binary(&res).unwrap();

        assert_eq!(counter, Uint128::new(1));
    }

    #[test]
    fn get_config() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);
        let env = mock_env();

        let msg = InstantiateMsg {};

        // instantiating
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "instantiate")]);

        // get config
        let msg = QueryMsg::GetConfig {};
        let res = query(deps.as_ref(), env, msg).unwrap();
        let _config: Config = from_binary(&res).unwrap();
    }
}