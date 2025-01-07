use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

declare_id!("BTcQwA3QqmkzaqTAMiKcnwAVNRpSNP1KdMUN5dXNLEUg");
#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, initializer_amount, taker_amount)?;
        ctx.accounts.deposit(initializer_amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    //   // Function to claim funds after timeout  
    //   pub fn claim_funds(ctx: Context<ClaimFunds>) -> Result<()> {  
    //     let escrow = &mut ctx.accounts.escrow;  

    //     // Assume some verification of timeout here  
    //     if Clock::get()?.unix_timestamp < escrow.timeout {  
    //         return Err(ErrorCode::ClaimFailed.into());  
    //     }  
        
    //     ctx.accounts.refund_and_close_vault()?;  
    //     emit!(FundsClaimed {});  
    //     Ok(())  
    // }  
}
