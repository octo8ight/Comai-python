use anchor_lang::prelude::*;

declare_id!("CRd5bDd42PyPPnKszYqdJLpmrQ7MKRDTXbmoVUVxeUVQ");

#[program]
pub mod solana_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
