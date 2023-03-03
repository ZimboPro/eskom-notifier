use eskom_se_push_api::errors::{HttpError, APIError};



pub fn map_error(err: HttpError) -> Option<String>{
    match err {
        HttpError::APIError(APIError::Forbidden) => {
          Some("The API key is invalid.".to_owned())
        }
        HttpError::Timeout => Some("The API call timed out.".to_owned()),
        HttpError::NoInternet => Some("No internet access.".to_owned()),
        HttpError::Unknown => Some("An error occurred".to_owned()),
        HttpError::ResponseError(_) => Some("An error occurred".to_owned()),
        HttpError::APIError(_) => Some("An error occurred.".to_owned()),
        HttpError::UreqResponseError(_) => Some("An error occurred".to_owned()),
        HttpError::SearchTextNotSet => Some("An error occurred".to_owned()),
        HttpError::AreaIdNotSet => Some("An error occurred".to_owned()),
        HttpError::LongitudeOrLatitudeNotSet {
          longitude: _,
          latitude: _,
        } => Some("An error occurred".to_owned()),
        HttpError::UnknownError(_) => Some("An error occurred".to_owned()),
      }
}