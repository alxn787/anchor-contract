use anchor_lang::prelude::*;

declare_id!("EDZMdYr4E4LbHJpSBAYXmXCPZYWeGB31fMJ7mp7yTj4t");

#[program]
pub mod fanch {
    use super::*;

    pub fn init(ctx:Context<Initialize>, init_value:u32)->Result<()>{
        ctx.accounts.account.num = init_value;
        Ok(())
    }

    pub fn add(ctx:Context<Add>, num:u32)->Result<()>{
        ctx.accounts.account.num = ctx.accounts.account.num + num;
        Ok(())
    }
     pub fn double(ctx:Context<Double>)->Result<()>{
        ctx.accounts.account.num = ctx.accounts.account.num * 2;
        Ok(())
     }
}

#[account]
struct DataShape{
    pub num:u32
}

#[derive(Accounts)] 
pub struct Initialize<'info>{
    #[account(init,payer = signer,space = 8+4)]
    pub account: Account<'info, DataShape>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    signer: Signer<'info>
}

#[derive(Accounts)] 
pub struct Double<'info>{
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    signer: Signer<'info>
}

#[derive(Accounts)] 
pub struct Add<'info>{
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    signer: Signer<'info>
}


