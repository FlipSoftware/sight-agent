use handle_errors::Error;
use std::collections::HashMap;

/// # Pagination `struct` reads the url query and returns based on input.
///
/// - Example
/// ```rust
/// fn get_page_offset(params: HashMap<String, String>) -> Result<Pagination, Error> {
///     if params.contains_key("key") {
///         Ok(// do something with the value captured and return a Result<Pagination>)
///         Err(// didn't capture expected value return an Result<Error>)
///     }
/// }
///
/// ```
#[derive(Default, Debug)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn get_page_offset(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("start")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::ParamsAbsent)
}
