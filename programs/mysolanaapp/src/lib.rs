use anchor_lang::prelude::*;

declare_id!("AstkrL2PDwKv6zvX6SWTxiJjDefawzpAMF3nuVZxBEwc");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
