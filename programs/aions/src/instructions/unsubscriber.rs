use std::ops::DerefMut;

use crate::constant::{CONTROLLER_SEED, SUBCRIBER_SEED};
use crate::state::{AionsStoraget, Controller};
use anchor_lang::prelude::*;
// use spl_account_compression::cpi as spl_ac_cpi;
use spl_account_compression::{program::SplAccountCompression, Noop};

#[derive(Accounts)]
pub struct UnSubscriber<'info> {
    #[account(
        mut,
        seeds = [SUBCRIBER_SEED, payer.key().as_ref()],
        bump,

    )]
    pub subscribers: Account<'info, AionsStoraget>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn unsubscriber_insctruction(ctx: Context<UnSubscriber>, wallet: Pubkey) -> Result<()> {
    let subscriber_acocunt = ctx.accounts.subscribers.deref_mut();

    let mut subscriber = subscriber_acocunt.subscriber.clone();
    if let Some(position) = subscriber.iter().position(|w| *w == wallet) {
        subscriber.remove(position);
    }

    *subscriber_acocunt = AionsStoraget {
        subscriber,
        ..subscriber_acocunt.clone()
    };
    Ok(())
}
