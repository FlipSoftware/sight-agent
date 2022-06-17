use tracing::{event, instrument, Level};
use warp::{body::BodyDeserializeError, reject::Reject, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    KBAbsent,
    ParamsAbsent,
    DBQueryError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Invalid parameter: {}", err),
            Error::ParamsAbsent => write!(f, "Missing required parameter"),
            Error::KBAbsent => write!(f, "KB not found in the database"),
            Error::DBQueryError => write!(f, "The typed data doesn't match the database format"),
        }
    }
}
impl Reject for Error {}

#[instrument]
pub async fn handle_errors(error: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = error.find::<Error>() {
        event!(Level::ERROR, "Error: {}", e);
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(e) = error.find::<warp::filters::cors::CorsForbidden>() {
        event!(Level::ERROR, "Error with CORS: fobbiden access");
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(_e) = error.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Error querying data from the database");
        Ok(warp::reply::with_status(
            crate::Error::DBQueryError.to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::ERROR, "Inaccessible route");
        Ok(warp::reply::with_status(
            "Inacessible route".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
