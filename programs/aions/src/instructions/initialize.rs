use crate::constant::{CONTROLLER_SEED, SUBCRIBER_SEED};
use crate::state::{AionsStoraget, Controller};
use anchor_lang::prelude::*;
use spl_account_compression::cpi as spl_ac_cpi;
use spl_account_compression::{program::SplAccountCompression, Noop};

pub fn initialize_insctuction(
    ctx: Context<Initialize>,
    max_depth: u32,
    max_buffer_size: u32,
) -> Result<()> {
    let accounts = spl_ac_cpi::accounts::Initialize {
        merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        authority: ctx.accounts.tree_controller.to_account_info(),
        noop: ctx.accounts.noop_program.to_account_info(),
    };

    let signer_seeds: &[&[&[u8]]] = &[&[
        CONTROLLER_SEED,
        &[*ctx.bumps.get("tree_controller").unwrap()],
    ]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.compression_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    spl_ac_cpi::init_empty_merkle_tree(cpi_ctx, max_depth, max_buffer_size)?;

    ctx.accounts.tree_controller.set_inner(Controller {
        authority: ctx.accounts.authority.key(),
        merkle_tree: ctx.accounts.merkle_tree.key(),
    });

    // set an empty Subscriber
    ctx.accounts.subscribers.set_inner(AionsStoraget {
        subscriber: vec![],
        current_message_index: 0,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: first call to initialize is permissionless
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    pub authority: Signer<'info>,

    // every user has a controller account for they are content
    #[account(
        init,
        space = 8 + 32 + 32,
        payer = payer,
        seeds = [CONTROLLER_SEED, payer.key().as_ref()],
        bump,
    )]
    pub tree_controller: Account<'info, Controller>,

    #[account(
        init,
        space = 8 + AionsStoraget::INIT_SPACE,
        payer = payer,
        seeds = [SUBCRIBER_SEED, payer.key().as_ref()],
        bump,
    )]
    pub subscribers: Account<'info, AionsStoraget>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
}
