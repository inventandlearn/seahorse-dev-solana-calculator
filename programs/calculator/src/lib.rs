use anchor_lang::prelude::*;

declare_id!("DdMkrf3qEZxymph9RSnXuZ4iHQJV6MJG9JiLr9vCA4fL");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
