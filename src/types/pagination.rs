use handle_errors::Error;
use std::collections::HashMap;

/// # Pagination `struct` reads the url query and returns based on input.
///
/// - Example
/// ```rust
/// fn get_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
///     if params.contains_key("key") {
///         Ok(// do something with the value captured and return a Result<Pagination>)
///         Err(// didn't capture expected value return an Result<Error>)
///     }
/// }
///
/// ```
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn get_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::ParamsAbsent)
}
