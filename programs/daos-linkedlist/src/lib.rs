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

// account to track the head
#[account]
#[derive(Default)]
pub struct LinkedList {
    pub head: Option<Pubkey>,
}

// node
#[account]
#[derive(Default)]
pub struct Node {
    pub data: u8,
    pub next: Option<Pubkey>,
}

// initialize
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<LinkedList>()
    )]
    pub linked_list: Account<'info, LinkedList>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// insert
#[derive(Accounts)]
pub struct Insert<'info> {
    #[account(mut)]
    pub linked_list: Account<'info, LinkedList>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Node>()
    )]
    pub node: Account<'info, Node>,

    #[account(mut)]
    pub after_node: Option<Account<'info, Node>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// delete
#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub linked_list: Account<'info, LinkedList>,

    #[account(mut)]
    pub node_to_delete: Account<'info, Node>,

    #[account(mut)]
    pub prev_node: Option<Account<'info, Node>>,

    pub payer: Signer<'info>,
}
