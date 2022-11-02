#[cfg(test)]
use crate::contract::{execute, instantiate, migrate, query};
#[cfg(test)]
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
#[cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
#[cfg(test)]
use cosmwasm_std::{attr, coin, from_binary, Response};

pub const ADDR1: &str = "legit_admin_address";
pub const ADDR2: &str = "incorrect_address";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}
#[test]
fn test_instantiate_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR2, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _err = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn migrate_works() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    let msg = MigrateMsg{};
    let _res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();
}