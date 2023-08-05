use multiversx_sc::{codec::multi_types::OptionalValue, types::Address};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, rust_biguint, whitebox_legacy::*, DebugApi,
};
use staking_contract::*;
//use multiversx_sc_scenario::num_bigint::BigUint;
use multiversx_sc::types::BigUint;


const WASM_PATH: &'static str = "output/staking-contract.wasm";
const USER_BALANCE: u64 = 1_000_000_000_000_000_000;

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> staking_contract::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<staking_contract::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> staking_contract::ContractObj<DebugApi>,
{
    pub fn new(sc_builder: ContractObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_zero);
        let user_address = b_mock.create_user_account(&rust_biguint!(USER_BALANCE));
        let sc_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner_address), sc_builder, WASM_PATH);

        // simulate deploy
        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        ContractSetup {
            b_mock,
            owner_address,
            user_address,
            contract_wrapper: sc_wrapper,
        }
    }
}

use multiversx_sc::contract_base::ContractBase;

#[test]
fn stake_unstake_test() {
    let mut setup = ContractSetup::new(staking_contract::contract_obj);
    let user_addr = setup.user_address.clone();

    setup
        .b_mock
        .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    // stake full
    setup
        .b_mock
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    StakingPosition {
                        stake_amount: managed_biguint!(USER_BALANCE),
                        last_action_block: 0,
                        reward_balance: managed_biguint!(0),
                    }
                );
            },
        )
        .assert_ok();

    setup
        .b_mock
        .check_egld_balance(&user_addr, &rust_biguint!(0));
    setup.b_mock.check_egld_balance(
        setup.contract_wrapper.address_ref(),
        &rust_biguint!(USER_BALANCE),
    );

    // unstake full
    setup
        .b_mock
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unstake(OptionalValue::None);

                assert!(sc
                    .staking_position(&managed_address!(&user_addr))
                    .is_empty());
            },
        )
        .assert_ok();

    setup
        .b_mock
        .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    
}

#[test]
fn rewards_per_seconds_test() {
    let mut setup = ContractSetup::new(staking_contract::contract_obj);
    let user_addr = setup.user_address.clone();

    // Stake full balance
    setup
        .b_mock
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();
            },
        )
        .assert_ok();

    // Set an initial timestamp
    let initial_timestamp = 0;
    setup
        .b_mock
        .set_block_timestamp(initial_timestamp);

    // Calculate rewards for a certain amount of time
    let time_passed = 1000; // Simulate 1000 seconds passing
    let final_timestamp = initial_timestamp + time_passed;
    setup
        .b_mock
        .set_block_timestamp(final_timestamp);

    // rewards per second
    setup
    .b_mock
    .execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
        let expected_rewards = BigUint::from(REWARDS_PER_SECOND) * BigUint::from(time_passed);
        assert_eq!(staking_pos.reward_balance, expected_rewards);
    })
    .assert_ok();
}

#[test]
fn rewards_test() {
    let mut setup = ContractSetup::new(staking_contract::contract_obj);
    let user_addr = setup.user_address.clone();

    // Stake full balance
    setup
        .b_mock
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();
            },
        )
        .assert_ok();

    // Set an initial timestamp
    let initial_timestamp = 0;
    setup
        .b_mock
        .set_block_timestamp(initial_timestamp);

    // Calculate rewards for a certain amount of time
    let time_passed = 1000; // Simulate 1000 seconds passing
    let final_timestamp = initial_timestamp + time_passed;
    setup
        .b_mock
        .set_block_timestamp(final_timestamp);

    // Claim rewards
    setup
        .b_mock
        .execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
            sc.claim_rewards();

            let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
            println!("Staking Position after claiming rewards: {:?}", staking_pos);           
            

        })
        .assert_ok();
}



