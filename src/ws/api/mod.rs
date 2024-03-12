use super::{ApiRequest, PlayerMap};

mod sub;

pub fn execute(request: ApiRequest, caller: i32, players: &PlayerMap) -> Result<(), ApiError> {
    match request.api.as_ref() {
        "playerSubscriptions/v1/update" => sub::v1::update(request, caller, players),
        // TODO handle heartbeat (if needed)
        "heartbeat" => Ok(()),
        _ => Err(ApiError::UnknownPath)
    }
}

#[derive(Debug)]
pub enum ApiError {
    UnknownPath,
    InvalidParams,
    NoParams,
    CallerNotFound,
}
