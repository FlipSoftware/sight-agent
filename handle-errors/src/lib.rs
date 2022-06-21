#![warn(clippy::all)]
use tracing::{event, instrument, Level};
use warp::{reject::Reject, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    KBMissing,
    ParamsAbsent,
    DBQueryError(sqlx::Error),
    WrongPassword,
    LibArgonError(argon2::Error),
    FailTokenDecryption,
    Unauthorized,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(ref err) => write!(f, "Invalid parameter: {}", err),
            Error::ParamsAbsent => write!(f, "Missing required parameter"),
            Error::KBMissing => write!(f, "KB not found in the database"),
            Error::DBQueryError(_) => write!(f, "Unable to proceed with the query"),
            Error::WrongPassword => write!(f, "Wrong password"),
            Error::LibArgonError(_) => write!(f, "Unable to verify password"),
            Error::FailTokenDecryption => write!(f, "Unable decrypt token"),
            Error::Unauthorized => write!(f, "Unauthorized access to modify content"),
        }
    }
}
impl Reject for Error {}

#[instrument]
pub async fn handle_errors(rejection: Rejection) -> Result<impl Reply, Rejection> {
    match rejection.find() {
        Some(crate::Error::DBQueryError(err)) => {
            event!(Level::ERROR, "Database query error");

            match err {
                sqlx::Error::Database(e) => {
                    if e.code().unwrap().parse::<i32>().unwrap() == 23505 {
                        Ok(warp::reply::with_status(
                            "Account already exists",
                            warp::http::StatusCode::CONFLICT,
                        ))
                    } else {
                        Ok(warp::reply::with_status(
                            "Unprocessable entity",
                            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
                        ))
                    }
                }
                _ => Ok(warp::reply::with_status(
                    "Unprocessable entity",
                    warp::http::StatusCode::UNPROCESSABLE_ENTITY,
                )),
            }
        }
        Some(crate::Error::WrongPassword) => {
            event!(Level::ERROR, "Invalid password");
            Ok(warp::reply::with_status(
                "Invalid email/password combination",
                warp::http::StatusCode::UNAUTHORIZED,
            ))
        }
        Some(crate::Error::LibArgonError(_)) => {
            event!(Level::ERROR, "Lib Argon error");
            Ok(warp::reply::with_status(
                "Unable to verify password",
                warp::http::StatusCode::UNAUTHORIZED,
            ))
        }
        Some(crate::Error::FailTokenDecryption) => {
            event!(Level::ERROR, "Fail token decryption");
            Ok(warp::reply::with_status(
                "Unable to decrypt token",
                warp::http::StatusCode::UNAUTHORIZED,
            ))
        }
        Some(crate::Error::Unauthorized) => {
            event!(Level::ERROR, "Unauthorized access");
            Ok(warp::reply::with_status(
                "Unauthorized access to modify content",
                warp::http::StatusCode::UNAUTHORIZED,
            ))
        }
        _ => Ok(warp::reply::with_status(
            "Unprocessable entity",
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        )),
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
