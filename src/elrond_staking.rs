#![no_std]
multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self, staking_amount: BigUint) {
        require!(staking_amount > BigUint::from(0u64), "Must stake more than 0");

        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount >= staking_amount, "Insufficient payment amount");

        let caller = self.blockchain().get_caller();

        // Calculate rewards based on global speed
        let global_speed = BigUint::from(300_000_000_000_000u64); 
        let elapsed_time = self.blockchain().get_block_timestamp().clone();
        let rewards = staking_amount.clone() * elapsed_time * global_speed;

        // Update staking position with rewards
        let stake_mapper = self.staking_position(&caller);
        let _staked_amount = stake_mapper.get();
        stake_mapper.update(|current_amount| *current_amount += staking_amount + rewards);

        // Update staked addresses
        self.staked_addresses().insert(caller);
    }

    #[endpoint]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        let caller = self.blockchain().get_caller();
        let stake_mapper = self.staking_position(&caller);

        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => stake_mapper.get(),
        };

        require!(unstake_amount > BigUint::from(0u64), "Invalid unstake amount");

        // Calculate rewards based on global speed
        let global_speed = BigUint::from(300_000_000_000_000u64); 
        let elapsed_time = self.blockchain().get_block_timestamp().clone();
        let rewards = unstake_amount.clone() * elapsed_time * global_speed;

        let remaining_stake = stake_mapper.update(|staked_amount| {
            require!(
                unstake_amount <= *staked_amount,
                "Insufficient staked amount"
            );
            *staked_amount -= &unstake_amount;

            staked_amount.clone()
        });

        if remaining_stake == BigUint::from(0u64) {
            self.staked_addresses().swap_remove(&caller);
        }

        // Transfer rewards and unstaked EGLD to the caller
        self.send().direct_egld(&caller, &(&rewards + &unstake_amount));
    }

    #[endpoint]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();

        // Calculate rewards based on global speed
        let global_speed = BigUint::from(300_000_000_000_000u64); 
        let elapsed_time = self.blockchain().get_block_timestamp().clone();
        let staking_position = self.staking_position(&caller).get();
        let rewards = staking_position * elapsed_time * global_speed;

        // Update staking position to deduct claimed rewards
        let stake_mapper = self.staking_position(&caller);
        stake_mapper.update(|current_amount| {
            let claimed_rewards = (*current_amount).clone() - rewards.clone();
            require!(claimed_rewards >= BigUint::from(0u64), "No rewards to claim");
            *current_amount = claimed_rewards;
        });

        // Transfer claimed rewards to the caller
        self.send().direct_egld(&caller, &rewards);
    }

    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

}