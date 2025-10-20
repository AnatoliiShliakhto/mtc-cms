#[derive(thiserror::Error, Debug)]
pub enum SmtpError {
    #[error("SMTP message error: {0}")]
    SmtpMessageError(#[from] lettre::error::Error),
    #[error("SMTP message address error: {0}")]
    SmtpMessageAddressError(#[from] lettre::address::AddressError),
    #[error("SMTP transport error: {0}")]
    SmtpTransportError(#[from] lettre::transport::smtp::Error),
}