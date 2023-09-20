use crate::constant::{CONTROLLER_SEED, SUBCRIBER_SEED};
use crate::state::{AionsStoraget, Controller};
use anchor_lang::prelude::*;
// use spl_account_compression::cpi as spl_ac_cpi;
use spl_account_compression::{program::SplAccountCompression, Noop};

#[derive(Accounts)]
pub struct RemoveMessage<'info> {
    /// CHECK: key is checked
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,

    pub authority: Signer<'info>,

    // every user has a controller account for they are content
    #[account(
        seeds = [CONTROLLER_SEED, payer.key().as_ref()],
        bump,
        has_one = merkle_tree,
    )]
    pub tree_controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [SUBCRIBER_SEED, payer.key().as_ref()],
        bump,

    )]
    pub subscribers: Account<'info, AionsStoraget>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
}

pub fn remove_message_insctuction(ctx: Context<RemoveMessage>, index: u64) -> Result<()> {
    Ok(())
}
