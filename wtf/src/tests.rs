#[cfg(test)]
use crate::contract::{execute, instantiate, migrate, query};
#[cfg(test)]
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, ContractStatusResponse, WinningBidResponse, MyBidResponse};
#[cfg(test)]
use cosmwasm_std::Uint128;
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
        admin: Some(ADDR1.to_string()),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}
#[test]
fn migrate_works() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: Some(ADDR1.to_string()),
    };
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    let msg = MigrateMsg{};
    let _res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();
}
#[test]
fn test_bid_functionalities() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: Some(ADDR1.to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //add bid
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::AddBid{};
    let _res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap();
    //test second bid
    let info = mock_info(ADDR2, &[coin(2_000_000, "ujunox")]);
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //query winning bid
    let msg = QueryMsg::WinningBid {  };
    let bin = query(deps.as_ref(), env.clone(), info.clone(), msg).unwrap();
    let res: WinningBidResponse = from_binary(&bin).unwrap();
    assert_eq!(res.winning, Uint128::from(2000000u64));
    //query contract status
    let msg = QueryMsg::ContractStatus {  };
    let bin = query(deps.as_ref(), env.clone(), info.clone(), msg).unwrap();
    let res: ContractStatusResponse = from_binary(&bin).unwrap();
    assert_eq!(res.status, true);
    //query account bid
    let msg = QueryMsg::MyBid {  };
    let bin = query(deps.as_ref(), env, info, msg).unwrap();
    let res: MyBidResponse = from_binary(&bin).unwrap();
    assert_eq!(res.bid, Uint128::from(2000000u64));
}
#[test]
fn test_withdraw_restrictions() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: Some(ADDR1.to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //add bid
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::AddBid{};
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    //withdraw funds should fail since contract still open
    let msg = ExecuteMsg::Retrieve { friend: None };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
}