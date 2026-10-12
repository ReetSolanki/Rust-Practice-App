use anchor_lang::prelude::*;

declare_id!("4Qf5TaZhu56pRb9gJ5Yj6nwfZeDKj1XBcwHHqnZ268Jr");

#[program]
pub mod calci_anch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
