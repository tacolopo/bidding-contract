//TO DO:
// 1)
use cosmwasm_std::{coin, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_binary, StdError, Uint128};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, WinningBidResponse, MyBidResponse, ContractStatusResponse};
use crate::state::{Config, CONFIG, DONATION_COUNT, OPEN, WINNING};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRYPTO: &str = "ujunox";
const SEND: &str = "juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4";

//trying to show different ways to do things to demonstrate understanding of material
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //match the optional admin passed: if there is one, it is stored, if not, the sender is stored
    match msg.admin {
        Some(admin) => {
            set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
            let validated_admin = deps.api.addr_validate(&admin)?;
            let config = Config {
                admin: validated_admin.clone(),
            };
            CONFIG.save(deps.storage, &config)?;
            WINNING.save(deps.storage, &Uint128::from(0u64))?;
            OPEN.save(deps.storage, &true)?;
            Ok(Response::new()
                .add_attribute("action", "instantiate")
                .add_attribute("admin", validated_admin.to_string()))
        }
        None => {
            set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
            let admin = info.sender.to_string();
            let validated_admin = deps.api.addr_validate(&admin)?;
            let config = Config {
                admin: validated_admin.clone(),
            };
            CONFIG.save(deps.storage, &config)?;
            WINNING.save(deps.storage, &Uint128::from(0u64))?;
            OPEN.save(deps.storage, &true)?;
            Ok(Response::new()
                .add_attribute("action", "instantiate")
                .add_attribute("admin", validated_admin.to_string()))
        }
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddBid {} => execute_add_bid(deps, env, info),
        ExecuteMsg::Close {} => execute_close(deps, env, info),
        ExecuteMsg::Retrieve { friend } => execute_retrieve(deps, env, info, friend),
    }
}
fn execute_add_bid(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let contract_state = OPEN.load(deps.storage)?;
    if !contract_state {
        return Err(ContractError::ContractClosed {});
    }
    let x = WINNING.load(deps.storage)?;
    for coin in info.funds {
        if coin.denom == CRYPTO {
            let donator = DONATION_COUNT.may_load(deps.storage, info.sender.clone())?;
            match donator {
                Some(donator) => {
                    let new_donation_count = donator + coin.amount;
                    if new_donation_count <= x {
                        return Err(ContractError::BidTooLow {});
                    } else {
                        DONATION_COUNT.save(
                            deps.storage,
                            info.sender.clone(),
                            &new_donation_count,
                        )?;
                        WINNING.save(deps.storage, &new_donation_count)?;
                    }
                }
                None => {
                    if coin.amount <= x {
                        return Err(ContractError::BidTooLow {});
                    } else {
                        DONATION_COUNT.save(deps.storage, info.sender.clone(), &coin.amount)?;
                        WINNING.save(deps.storage, &coin.amount)?;
                    }
                }
            }
        } else {
            continue;
        }
    }
    //FLAT FEE
    let share = BankMsg::Send {
        to_address: SEND.to_string(),
        amount: vec![coin(1, CRYPTO)],
    };
    Ok(Response::new().add_message(share))
}
fn execute_close(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    OPEN.save(deps.storage, &false)?;
    Ok(Response::new())
}
fn execute_retrieve(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    friend: Option<String>,
) -> Result<Response, ContractError> {
    let state = OPEN.load(deps.storage)?;
    if state == true {
        return Err(ContractError::CannotWithdraw {  });
    }
    let check = DONATION_COUNT.load(deps.storage, info.sender.clone())?;
    match friend {
        Some(friend) => {
            let share = BankMsg::Send {
                to_address: friend,
                amount: vec![coin(u128::from(check), CRYPTO)],
            };
            Ok(Response::new().add_message(share))
        }
        None => {
            let share = BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: vec![coin(u128::from(check), CRYPTO)],
            };
            Ok(Response::new().add_message(share))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, info: MessageInfo, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractStatus {} => query_contract_status(deps, env),
        QueryMsg::MyBid {} => query_my_bid(deps, env, info),
        QueryMsg::WinningBid {} => query_winning_bid(deps, env),
    }
}
fn query_contract_status(deps: Deps, _env: Env) -> StdResult<Binary> {
    let status = OPEN.load(deps.storage)?;
    to_binary(&ContractStatusResponse { status })
}
fn query_my_bid(deps: Deps, _env: Env, info: MessageInfo) -> StdResult<Binary> {
    let bid = DONATION_COUNT.load(deps.storage, info.sender)?;
    to_binary(&MyBidResponse { bid })
}
fn query_winning_bid(deps: Deps, _env: Env) -> StdResult<Binary> {
    let winning = WINNING.load(deps.storage)?;
    to_binary(&WinningBidResponse { winning })
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    #[allow(clippy::cmp_owned)]
    if ver.version > (*CONTRACT_VERSION).to_string() {
        return Err(StdError::generic_err("Must upgrade from a lower version").into());
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default()
        .add_attribute("action", "migration")
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("contract", CONTRACT_NAME))
}
