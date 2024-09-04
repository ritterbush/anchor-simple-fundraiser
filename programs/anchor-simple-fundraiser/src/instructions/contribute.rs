use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{
    state::{
        Contributor,
        Fundraiser,
        Vault,
    },
    FundraiserError,
    ANCHOR_DISCRIMINATOR,
};

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(
        mut,
        seeds = [b"fundraiser".as_ref(), fundraiser.maker.as_ref()],
        bump = fundraiser.bump,
    )]
    pub fundraiser: Account<'info, Fundraiser>,
    #[account(
        init_if_needed,
        payer = contributor,
        seeds = [b"contributor", fundraiser.key().as_ref(), contributor.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR + Contributor::INIT_SPACE,
    )]
    pub contributor_account: Account<'info, Contributor>,
    #[account(
        mut,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl<'info> Contribute<'info> {
    pub fn contribute(&mut self, amount: u64) -> Result<()> {

        // Check if the fundraising duration has been reached
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            self.fundraiser.deadline >= current_time,
            FundraiserError::FundraiserEnded
        );

        // Transfer the funds to the vault
        // CPI to the token program to transfer the funds
        let cpi_program = self.system_program.to_account_info();

        // Transfer the funds from the contributor to the vault
        let cpi_accounts = Transfer {
            from: self.contributor_account.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // Crete a CPI context
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Transfer the funds from the contributor to the vault
        transfer(cpi_ctx, amount)?;

        // Update the fundraiser and contributor accounts with the new amounts
        self.fundraiser.current_amount += amount;

        self.contributor_account.amount += amount;

        Ok(())
    }
}
