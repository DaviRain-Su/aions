use crate::constant::{CONTROLLER_SEED, SUBCRIBER_SEED};
use crate::state::{AionsStoraget, Controller};
use anchor_lang::prelude::*;
use spl_account_compression::cpi as spl_ac_cpi;
use spl_account_compression::{program::SplAccountCompression, Noop};

#[derive(Accounts)]
pub struct AddNewMessage<'info> {
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

pub fn add_new_message_insctuction(
    ctx: Context<AddNewMessage>,
    hash_value: [u8; 32],
) -> Result<()> {
    let node = hash_value;

    let accounts = spl_ac_cpi::accounts::Modify {
        merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        authority: ctx.accounts.tree_controller.to_account_info(),
        noop: ctx.accounts.noop_program.to_account_info(),
    };

    let bump = *ctx.bumps.get("tree_controller").unwrap();
    let signer_seeds: &[&[&[u8]]] = &[&[CONTROLLER_SEED, &[bump]]];

    let cpi_ctx = CpiContext::new(ctx.accounts.compression_program.to_account_info(), accounts)
        .with_signer(signer_seeds);

    spl_ac_cpi::append(cpi_ctx, node)?;

    Ok(())
}
