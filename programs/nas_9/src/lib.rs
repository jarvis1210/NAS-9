use anchor_lang::prelude::*;

declare_id!("3zRWN1YxsE9TXFEFQGELDxfRXq947z63RuNodeNLvrmr");

#[program]
pub mod nas_9 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter.data = 0;
        ctx.accounts.counter.owner = ctx.accounts.owner.key();
        msg!("Hello world!");
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.data += 1;
        msg!("Counter incremented to {}", counter.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[b"NAS COUNTER", owner.key().as_ref()],
        bump,
        payer = owner,
        space=8+1+32
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = owner)]
    pub counter: Account<'info, Counter>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Counter {
    pub data: u8, //1 byte
    pub owner: Pubkey, //32 bytes
}
