#![cfg(test)]

use crate::{error::Error, DisputeRegistryContract, DisputeRegistryContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn client<'a>(env: &'a Env) -> (DisputeRegistryContractClient<'a>, Address, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let executor = Address::generate(env);
    let id = env.register(DisputeRegistryContract, (admin.clone(), executor.clone()));
    (
        DisputeRegistryContractClient::new(env, &id),
        admin,
        executor,
    )
}

#[test]
fn constructor_stores_admin_and_executor() {
    let env = Env::default();
    let (client, admin, executor) = client(&env);
    let cfg = client.get_config();
    assert_eq!(cfg.admin, admin);
    assert_eq!(cfg.executor, executor);
}

#[test]
fn week1_writes_are_declared_but_not_implemented() {
    let env = Env::default();
    let (client, _, _) = client(&env);
    let judge = Address::generate(&env);
    let err = client.try_register_judge(&judge).unwrap_err().unwrap();
    assert_eq!(err, Error::NotImplemented);
}
