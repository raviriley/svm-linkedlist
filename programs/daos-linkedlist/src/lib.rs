use anchor_lang::prelude::*;

declare_id!("HQoaeoSsmnjZBS7UvxDvGkRi6HacT18oCrskAm2oatE4");

#[program]
pub mod daos_linkedlist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
