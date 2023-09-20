#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

pub mod constant;
pub mod instructions;
pub mod state;

pub use instructions::*;

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

#[program]
pub mod aions {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        initialize_insctuction(ctx, max_depth, max_buffer_size)
    }

    // add new messsage
    pub fn add_new_message(ctx: Context<AddNewMessage>, hash_value: [u8; 32]) -> Result<()> {
        add_new_message_insctuction(ctx, hash_value)
    }

    // remove message
    pub fn remove_message(ctx: Context<RemoveMessage>, index: u64) -> Result<()> {
        remove_message_insctuction(ctx, index)
    }

    pub fn update_message(ctx: Context<UpdateMessage>, index: u64) -> Result<()> {
        update_message_insctuction(ctx, index)
    }

    pub fn subscriber(ctx: Context<Subscriber>, wallet: Pubkey) -> Result<()> {
        subscriber_insctruction(ctx, wallet)
    }

    pub fn unsubscriber(ctx: Context<UnSubscriber>, wallet: Pubkey) -> Result<()> {
        unsubscriber_insctruction(ctx, wallet)
    }
}
