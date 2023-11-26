use std::fmt;
use std::fmt::Formatter;
use once_cell::sync::Lazy;
use thiserror::Error;


#[allow(dead_code)]
#[derive(Error, Debug)]
pub(crate) enum Errors {
    InvalidCookies(InvalidCookies),
    InvalidLanguage(InvalidLanguage),
    VisitsTooFrequently(VisitsTooFrequently),
    MalformedRequest(MalformedRequest),
    NoAccAssociatedCookies(NoAccAssociatedCookies),
    TooManyRequests(TooManyRequests),
    DataNotPublic(DataNotPublic),
    InternalDatabaseError(InternalDatabaseError),
    AccountNotFound(AccountNotFound),
    RedemptionInvalid(RedemptionInvalid),
    RedemptionCooldown(RedemptionCooldown),
    RedemptionClaimed(RedemptionClaimed),
    AlreadyClaimed(RedemptionClaimed),
    AccountLoginFail(AccountLoginFail),
    AccountHasLocked(AccountHasLocked),
}


impl fmt::Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Errors::InvalidCookies(_) => write!(f, "Cookies are not valid."),
            Errors::InvalidLanguage(_) => write!(f, "Language is not valid."),
            Errors::VisitsTooFrequently(_) => write!(f, "Visits too frequently."),
            Errors::MalformedRequest(_) => write!(f, "Malformed request."),
            Errors::NoAccAssociatedCookies(_) => write!(f, "No game account associated with cookies"),
            Errors::TooManyRequests(_) => write!(f, "Cannot get data for more than 30 accounts per cookie per day."),
            Errors::DataNotPublic(_) => write!(f, "User's data is not public."),
            Errors::InternalDatabaseError(_) => write!(f, "Internal database error."),
            Errors::AccountNotFound(_) => write!(f, "Couldn't find the account."),
            Errors::RedemptionInvalid(_) => write!(f, "Invalid redemption code."),
            Errors::RedemptionCooldown(_) => write!(f, "Redemption is on cooldown."),
            Errors::RedemptionClaimed(_) => write!(f, "Redemption code has been claimed already."),
            Errors::AlreadyClaimed(_) => write!(f, "Already claimed the daily reward today."),
            Errors::AccountLoginFail(_) => write!(f, "Account login failed."),
            Errors::AccountHasLocked(_) => write!(f, "Account has been locked because exceeded password limit. Please wait 20mins and try again."),
        }
    }
}


trait BaseError {}

#[derive(Debug)]
pub struct InvalidCookies;
impl BaseError for InvalidCookies {}

#[derive(Debug)]
pub struct InvalidLanguage;
impl BaseError for InvalidLanguage {}

#[derive(Debug)]
pub struct VisitsTooFrequently;
impl BaseError for VisitsTooFrequently {}

#[derive(Debug)]
pub struct MalformedRequest;
impl BaseError for MalformedRequest {}

#[derive(Debug)]
pub struct NoAccAssociatedCookies;
impl BaseError for NoAccAssociatedCookies {}

#[derive(Debug)]
pub struct TooManyRequests;
impl BaseError for TooManyRequests {}

#[derive(Debug)]
pub struct DataNotPublic;
impl BaseError for DataNotPublic {}

#[derive(Debug)]
pub struct InternalDatabaseError;
impl BaseError for InternalDatabaseError {}

#[derive(Debug)]
pub struct AccountNotFound;
impl BaseError for AccountNotFound {}

#[derive(Debug)]
pub struct RedemptionInvalid;
impl BaseError for RedemptionInvalid {}

#[derive(Debug)]
pub struct RedemptionCooldown;
impl BaseError for RedemptionCooldown {}

#[derive(Debug)]
pub struct RedemptionClaimed;
impl BaseError for RedemptionClaimed {}

#[derive(Debug)]
pub struct AlreadyClaimed;
impl BaseError for AlreadyClaimed {}

#[derive(Debug)]
pub struct AccountLoginFail;
impl BaseError for AccountLoginFail {}

#[derive(Debug)]
pub struct AccountHasLocked;
impl BaseError for AccountHasLocked {}


static ERRORS: Lazy<Vec<(i16, Errors)>> = Lazy::new(|| {
    vec![
        (-100, Errors::InvalidCookies(InvalidCookies {})),
        (-108, Errors::InvalidLanguage(InvalidLanguage {})),
        (-110, Errors::VisitsTooFrequently(VisitsTooFrequently {})),

        (10001, Errors::InvalidCookies(InvalidCookies {})),
        (-10001, Errors::MalformedRequest(MalformedRequest {})),
        (-10002, Errors::NoAccAssociatedCookies(NoAccAssociatedCookies {})),

        (10101, Errors::TooManyRequests(TooManyRequests {})),
        (10102, Errors::DataNotPublic(DataNotPublic {})),
        // (10103, Errors::InvalidCookies()),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
        // (),
    ]
});


// pub(crate) fn check_has_error(response: reqwest::Response) -> anyhow::Result<reqwest::Response> {
//     let resp = &response;
// }