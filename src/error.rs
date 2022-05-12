//! Possible client errors that can occur when interacting with the API.

use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{Display, Formatter, Result},
    io::Error as IOError,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ureq::{Error as RequestError, Transport};

use super::APIVersion;

/// All possible errors that can occur when using the client.
#[derive(Debug, Error)]
pub enum ClientError {
    /// I/O Error.
    #[error("some kind of I/O error occured: `{0}`")]
    IOError(#[from] IOError),
    /// HTTP response error.
    #[error("HTTP response error: `{errors:?}`")]
    ResponseError {
        /// Response errors.
        errors: APIErrorResponse,
    },
    /// HTTP transport error.
    #[error("HTTP transport error: `{0}`")]
    TransportError(#[from] Transport),
    /// Invalid query parameter error.
    #[error("invalid query parameter: `{name}={value}`")]
    InvalidQueryParam {
        /// The name of the query parameter.
        name: String,
        /// The value of the query parameter.
        value: String,
    },
}

/// Custom error response from the MBTA API.
#[derive(Debug, Deserialize, Serialize)]
pub struct APIErrorResponse {
    /// API errors.
    pub errors: Vec<APIError>,
    /// API version.
    pub jsonapi: APIVersion,
}

impl Display for APIErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ errors: (")?;
        for error in &self.errors {
            write!(f, "{}, ", error)?;
        }
        write!(f, "), api version: {:?}}}", self.jsonapi)
    }
}

impl StdError for APIErrorResponse {}

/// Custom error from the MBTA API.
#[derive(Debug, Deserialize, Serialize)]
pub struct APIError {
    /// Error code.
    pub code: String,
    /// Error status.
    pub status: String,
    /// Error details.
    #[serde(default)]
    pub detail: Option<String>,
    /// Error source.
    #[serde(default)]
    pub source: Option<HashMap<String, String>>,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{code: {}, status: {}", self.code, self.status)?;
        if let Some(detail) = &self.detail {
            write!(f, ", detail: {}", detail)?;
        }
        if let Some(source) = &self.source {
            write!(f, ", source: {:?}", source)?;
        }
        write!(f, "}}")
    }
}

impl StdError for APIError {}

impl From<RequestError> for ClientError {
    fn from(error: RequestError) -> Self {
        match error {
            RequestError::Status(_, response) => {
                let errs = response.into_json::<APIErrorResponse>();
                match errs {
                    Ok(errors) => Self::ResponseError { errors },
                    Err(io) => Self::from(io),
                }
            }
            RequestError::Transport(err) => Self::from(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;
    use std::io::ErrorKind;
    use ureq::Response;

    #[rstest]
    fn test_client_error_display_io_error() {
        // Arrange
        let input = IOError::new(ErrorKind::BrokenPipe, "test error");
        let expected = format!("some kind of I/O error occured: `{}`", input);
        let error = ClientError::from(input);

        // Act
        let actual = format!("{}", error);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::valid_error_text(
        "{\"errors\": [{\"status\": \"403\", \"code\": \"forbidden\"}], \"jsonapi\": {\"version\": \"1.0\"}}",
        "HTTP response error: `APIErrorResponse { errors: [APIError { code: \"forbidden\", status: \"403\", detail: None, source: None }], jsonapi: APIVersion { version: \"1.0\" } }`",
    )]
    #[case::invalid_error_text(
        "foobar",
        "some kind of I/O error occured: `Failed to read JSON: expected ident at line 1 column 2`"
    )]
    fn test_client_error_display_request_error(#[case] text: &str, #[case] expected: &str) {
        // Arrange
        let input = RequestError::Status(404, Response::new(404, "Page not found", text).unwrap());
        let error = ClientError::from(input);

        // Act
        let actual = format!("{}", error);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_client_error_display_invalid_query_param_error() {
        // Arrange
        let error = ClientError::InvalidQueryParam {
            name: "foo".into(),
            value: "bar".into(),
        };
        let expected = format!("invalid query parameter: `foo=bar`");

        // Act
        let actual = format!("{}", error);

        // Assert
        assert_eq!(actual, expected);
    }
}
