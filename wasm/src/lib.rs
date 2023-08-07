// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           11
// Async Callback (empty):               1
// Total number of exported functions:  13

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    staking_contract
    (
        init => init
        stake => stake
        unstake => unstake
        claim_rewards => claim_rewards
        calculateRewardsForUser => calculate_rewards_for_user
        getContractBalance => get_contract_balance
        contractCreationBlock => contract_creation_block
        contractCreationTimestamp => contract_creation_timestamp
        getStakeAmount => get_stake_amount
        getUpdatedTotalRewards => get_updated_total_rewards
        getStakedAddresses => staked_addresses
        getStakingPosition => staking_position
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
