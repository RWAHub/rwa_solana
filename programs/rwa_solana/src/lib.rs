use anchor_lang::prelude::*;

declare_id!("3PJVuFionEzADePhZJ5u3h81q5d6EmQ8JiM2cqCp7zPA");

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
