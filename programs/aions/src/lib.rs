#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use spl_account_compression::cpi as spl_ac_cpi;
use spl_account_compression::{program::SplAccountCompression, Noop};

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "aions on solana core program",
    project_url: "https://github.com/DaviRain-Su",
    contacts: "email:davirian.yin@gmail.com",
    policy: "Please report (suspected) security vulnerabilities to email above.
You will receive a response from us within 48 hours.",
    source_code: "https://github.com/DaviRain-Su/aions",
    source_revision: env!("GIT_HASH"),
    acknowledgements: "Everyone in the Solana community"
}

declare_id!("Ht6pUzafXFPPf1TmNSNuYpfWkTZCRp3XJh8X5AJFisM4");

pub const CONTROLLER_SEED: &[u8] = b"aions-controller";

#[program]
pub mod aions {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        let accounts = spl_ac_cpi::accounts::Initialize {
            merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
            authority: ctx.accounts.controller.to_account_info(),
            noop: ctx.accounts.noop_program.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] =
            &[&[CONTROLLER_SEED, &[*ctx.bumps.get("controller").unwrap()]]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.compression_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        spl_ac_cpi::init_empty_merkle_tree(cpi_ctx, max_depth, max_buffer_size)?;

        ctx.accounts.controller.set_inner(Controller {
            authority: ctx.accounts.authority.key(),
            merkle_tree: ctx.accounts.merkle_tree.key(),
        });

        Ok(())
    }
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
    pub controller: Account<'info, Controller>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Controller {
    pub authority: Pubkey,
    pub merkle_tree: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Subscriber {
    // set max size subscribet number is 1000
    #[max_len(1000)]
    pub subscriber: Vec<Pubkey>,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_subscribet_size() {
        println!("Subscriber size: {}", Subscriber::INIT_SPACE);
    }
}
