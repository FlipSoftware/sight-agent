use warp::{body::BodyDeserializeError, reject::Reject, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    KbAbsent,
    ParamsAbsent,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Invalid parameter: {}", err),
            Error::ParamsAbsent => write!(f, "Missing required parameter"),
            Error::KbAbsent => write!(f, "KB not found in the database"),
        }
    }
}
impl Reject for Error {}

pub async fn handle_errors(error: Rejection) -> Result<impl Reply, Rejection> {
    println!("{error:?}");
    if let Some(e) = error.find::<Error>() {
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(e) = error.find::<warp::filters::cors::CorsForbidden>() {
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = error.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
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
