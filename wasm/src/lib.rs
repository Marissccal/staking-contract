// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   6

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    staking_contract
    (
        init => init
        stake => stake
        getStakingPosition => staking_position
        getTotalStakedAmount => total_staked_amount
        getTotalRewardsAmount => total_rewards_amount
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
