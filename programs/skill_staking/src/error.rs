use anchor_lang::error_code;

#[error_code]
pub enum DefiOSError {
    // 6000
    #[msg("Invalid Signature")]
    SignatureVerificationFailed,

    // 6001
    #[msg("User not verified")]
    UnauthorizedUser,

    //6002
    #[msg("Unauthorized smart contract Action")]
    UnauthorizedActionAttempted,
}
