use anchor_lang::prelude::*;

#[account]
pub struct Controller {
    pub authority: Pubkey,
    pub merkle_tree: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct AionsStoraget {
    // set max size subscribet number is 1000
    #[max_len(1000)]
    pub subscriber: Vec<Pubkey>,

    pub current_message_index: u64,

    #[max_len(1000)]
    pub remove_index: Vec<bool>,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_aions_storage_size() {
        println!("AionsStoraget size: {}", AionsStoraget::INIT_SPACE);
    }
}
