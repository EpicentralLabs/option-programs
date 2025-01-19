use anchor_lang::prelude::*;

declare_id!("9SaKv891pjh3YMtvUxL5jHQc77Gs6Ay4uy2PifX5AEPL");

#[program]
pub mod option_create {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
