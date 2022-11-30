//TO DO:
// 1) 
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, StdError};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, BankMsg};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Config, CONFIG, DONATION_COUNT};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRYPTO: &str = "ujunox";

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
        }
}
fn execute_add_bid(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    for coin in info.funds {
        if coin.denom == "uatom" {
            let donator = DONATION_COUNT.may_load(deps.storage, info.sender.clone())?;
            match donator {
                Some(donator) => {
                    let new_donation_count = donator + coin.amount;
                    DONATION_COUNT.save(deps.storage, info.sender.clone(), &new_donation_count)?;
                }
                None => {
                    DONATION_COUNT.save(deps.storage, info.sender.clone(), &coin.amount)?;
                }
            }
        }
        else { continue }
    }
    //update response
    Ok(Response::new())
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // QueryMsg::Example {} => query_msg_example(deps, env),
    }
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
