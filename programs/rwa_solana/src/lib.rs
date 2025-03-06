use anchor_lang::prelude::*;

declare_id!("56jqw6LPMx2U7LYQf8vSu2DHD3HPAVTniRunaE9FyFzb");

#[program]
pub mod rwa_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
