pub mod statistics;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusCode {
    Success = 200,
    NoChange = 201,
    DeprecatedMessageType = 300,
    Redirect = 301,
    Retry = 302,
    MalformedData = 400,
    UnsupportedEncoding = 401,
    ServerTemporarilyUnavailable = 420,
    ServerShuttingDownAtOperatorRequest = 421,
    AccessDenied = 430,
    AccoundSuspended = 431,
    AccountTerminated = 432,
    SyntaxErrorInRequest = 500,
    SyntaxErrorIlliegalParameters = 501,
    NotImplemented = 502,
    CommandParameterNotImplemented = 503,
    RequestTooBig = 504,
}

impl From<u16> for StatusCode {
    fn from(value: u16) -> Self {
        match value {
            200 => Self::Success,
            201 => Self::NoChange,
            300 => Self::DeprecatedMessageType,
            301 => Self::Redirect,
            302 => Self::Retry,
            400 => Self::MalformedData,
            401 => Self::UnsupportedEncoding,
            420 => Self::ServerTemporarilyUnavailable,
            421 => Self::ServerShuttingDownAtOperatorRequest,
            430 => Self::AccessDenied,
            431 => Self::AccoundSuspended,
            432 => Self::AccountTerminated,
            500 => Self::SyntaxErrorInRequest,
            501 => Self::SyntaxErrorIlliegalParameters,
            502 => Self::NotImplemented,
            503 => Self::CommandParameterNotImplemented,
            504 => Self::RequestTooBig,
            _ => panic!("Unknown status code: {}", value),
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Success
    }
}
trait Response {
    fn code(&self) -> StatusCode;
    fn status(&self) -> String;
}
