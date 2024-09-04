use anchor_lang::error_code;

#[error_code]
pub enum FundraiserError {
    #[msg("The amount to raise has not been met")]
    TargetNotMet,
    #[msg("The amount to raise has been achieved")]
    TargetMet,
    #[msg("The fundraiser has not ended yet")]
    FundraiserNotEnded,
    #[msg("The fundraiser has ended")]
    FundraiserEnded,
}
