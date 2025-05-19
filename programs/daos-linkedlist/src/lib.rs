use anchor_lang::prelude::*;

declare_id!("HQoaeoSsmnjZBS7UvxDvGkRi6HacT18oCrskAm2oatE4");

#[program]
pub mod daos_linkedlist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let linked_list = &mut ctx.accounts.linked_list;
        linked_list.head = None;
        Ok(())
    }

    pub fn insert(ctx: Context<Insert>, data: u8) -> Result<()> {
        let node = &mut ctx.accounts.node;
        let linked_list = &mut ctx.accounts.linked_list;

        node.data = data;

        // if there exists an after_node, we are inserting after a specific node and need to link the new node to the next node
        if let Some(after_node) = ctx.accounts.after_node.as_mut() {
            node.next = after_node.next;
            after_node.next = Some(node.key());
        } else {
            // Otherwise, we are inserting at the head
            node.next = linked_list.head;
            linked_list.head = Some(node.key());
        }

        Ok(())
    }

    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        let linked_list = &mut ctx.accounts.linked_list;
        let node_to_delete = &ctx.accounts.node_to_delete;

        // if removing the head, we need to update the head to the next node
        if let Some(head) = linked_list.head {
            if head == node_to_delete.key() {
                linked_list.head = node_to_delete.next;
                return Ok(());
            }
        }

        // if removing a node in the middle/end, we need to connect the previous node to the node after the one being deleted
        if let Some(prev_node) = ctx.accounts.prev_node.as_mut() {
            if let Some(next_key) = prev_node.next {
                if next_key == node_to_delete.key() {
                    prev_node.next = node_to_delete.next;
                }
            }
        }

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
