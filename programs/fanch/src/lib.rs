use anchor_lang::prelude::*;

declare_id!("EDZMdYr4E4LbHJpSBAYXmXCPZYWeGB31fMJ7mp7yTj4t");

#[program]
pub mod fanch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
