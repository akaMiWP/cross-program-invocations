use anchor_lang::prelude::*;

declare_id!("5oPMo78DeonzVXKezfD32MNCYT3YcsDPwVd3YHw1oTvU");

#[program]
pub mod game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
