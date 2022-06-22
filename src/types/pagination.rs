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
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn get_page_offset(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
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

#[cfg(test)]
mod pagination_tests {
    use color_eyre::Result;

    use super::*;

    #[test]
    fn valid_pagination() -> Result<()> {
        color_eyre::install()?;

        let mut params = HashMap::new();
        params.insert("limit".to_string(), "1".to_string());
        params.insert("offset".to_string(), "1".to_string());

        let page_result = get_page_offset(params);
        let valid_rhs = Pagination {
            limit: Some(1),
            offset: 1,
        };

        assert_eq!(page_result.unwrap(), valid_rhs);
        Ok(())
    }

    #[test]
    fn missing_offset_parameter() -> Result<()> {
        let mut params = HashMap::new();
        params.insert("limit".to_string(), "1".to_string());

        let page_result = format!("{}", get_page_offset(params).unwrap_err());
        let valid_rhs = format!("{}", Error::ParamsAbsent);

        assert_eq!(page_result, valid_rhs);
        Ok(())
    }
}
