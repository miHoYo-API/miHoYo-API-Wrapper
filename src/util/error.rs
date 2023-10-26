#[derive(thiserror::Error, Debug)]
pub enum APIErrorKind {
    #[error("")]
    InternalDatabaseError,
    #[error("Could not find user; uid may be invalid.")]
    AccountNotFound,
    #[error("User's data is not public.")]
    DataNotPublic,
    #[error("Cookies are not valid.")]
    InvalidCookies,
    #[error("Cannot get data for more than 30 accounts per cookie per day.")]
    TooManyRequests,
    #[error("Visits too frequently.")]
    VisitsTooFrequently,
    #[error("Already claimed the daily reward today.")]
    AlreadyClaimed,
    #[error("Geetest triggered during daily reward claim.")]
    GeetestTriggered,
    #[error("Authkey is not valid.")]
    InvalidAuthkey,
    #[error("Authkey has timed out.")]
    AuthkeyTimeout,
    #[error("Invalid redemption code.")]
    RedemptionInvalid,
    #[error("Redemption is on cooldown.")]
    RedemptionCooldown,
    #[error("Redemption code has been claimed already.")]
    RedemptionClaimed,
    #[error("Account login failed.")]
    AccountLoginFail,
    #[error("Account has been locked because exceeded password limit. Please wait 20 minute and try again")]
    AccountHasLocked,
    #[error("unknown data store error")]
    Unknown,
}


pub struct APIException {
    pub retcode: i16,
    pub original: String,
    pub msg: String,
}
impl APIException {
    pub fn response(&self) -> String {
        format!("retcode: {}, message: {}, data: None", self.retcode, self.original)
    }
}