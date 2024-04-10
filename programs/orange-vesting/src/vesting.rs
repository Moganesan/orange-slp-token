use anchor_lang::prelude::*;
use spl_token::solana_program::pubkey::Pubkey;

#[program]
mod vesting {
    use super::*;

    #[derive(Accounts)]
    pub struct ClaimToken<'info> {
        #[account(mut)]
        token_account: Account<'info, TokenAccount>,

        #[account(signer)]
        owner: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct InitializeVesting<'info> {
        #[account(mut)]
        pub token: Account<'info, Token>,

        pub owner: AccountInfo<'info>,
    }

    #[account]
    pub struct Token {
        pub total_supply: u64,
        // Add other token metadata fields as needed
    }

    #[derive(Accounts)]
    pub struct TokenAccount<'info> {
        #[account(mut)]
        pub token_account: Account<'info, Token>,
    }

    #[derive(Accounts)]
    pub struct TokenAllocation {
        pub team: u64,
        pub advisors: u64,
        pub liquidity: u64,
        pub ecosystem_rewards: u64,
        pub community_airdrop: u64,
    }

    #[derive(Accounts)]
    pub struct VestingDetails<'info> {
        #[account(mut)]
        pub team_member: Account<'info, TokenAccount>,
    }

    #[derive(Accounts)]
    pub struct ClaimTokenAccounts<'info> {
        #[account(mut)]
        pub team_member: Account<'info, TokenAccount>,

        #[account(mut)]
        pub vesting_details: Account<'info, VestingDetails<'info>>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct VestingParams {
        pub release_interval: u64,
        pub vesting_start_time: u64,
    }

    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        params: VestingParams,
        token_allocation: TokenAllocation,
    ) -> ProgramResult {
        let vesting = Vesting {
            release_interval: params.release_interval,
            vesting_start_time: params.vesting_start_time,
            token_allocation,
        };
        vesting.save(&mut ctx.accounts.vesting_data);
        Ok(())
    }

    pub fn claim_token(ctx: Context<ClaimToken>, area: String) -> ProgramResult {
        let vesting_data = &mut ctx.accounts.vesting_data.load_mut()?;

        let allocated_tokens = vesting_data
            .token_allocation
            .get(&area)
            .ok_or(ErrorCode::InvalidArea)?;

        // Implement the rest of the claim token logic here

        Ok(())
    }

    #[error]
    pub enum ErrorCode {
        #[msg("Invalid area specified")]
        InvalidArea,
    }
}

#[derive(Accounts)]
pub struct VestingData {
    pub vesting_data: AccountInfo<'info>,
}

pub struct Vesting {
    pub release_interval: u64,
    pub vesting_start_time: u64,
    pub token_allocation: TokenAllocation,
}

impl Vesting {
    pub fn save(&self, vesting_data: &mut VestingData) {
        // Implement saving vesting data to account
    }
}
