//! Possible client errors that can occur when interacting with the API. Note that this is different from an error response.

use std::{error::Error, fmt::Display, io::Error as IOError};

use serde_json::Error as JSONError;
use ureq::Error as RequestError;

/// All possible errors that can occur when using the client.
#[derive(Debug)]
pub enum ClientError {
    /// I/O Error.
    IOError(Box<IOError>),
    /// HTTP request error.
    RequestError(Box<RequestError>),
    /// Invalid query parameter error.
    InvalidQueryParam(String, String),
    /// JSON parsing error.
    JSONError(Box<JSONError>),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MBTA Client Error, ")?;
        match self {
            ClientError::IOError(i) => {
                write!(f, "IO error: ")?;
                i.fmt(f)
            }
            ClientError::RequestError(r) => {
                write!(f, "request error: ")?;
                r.fmt(f)
            }
            ClientError::InvalidQueryParam(k, v) => {
                write!(f, "query parameter error: {}={}", k, v)
            }
            ClientError::JSONError(j) => {
                write!(f, "JSON error: ")?;
                j.fmt(f)
            }
        }
    }
}

impl Error for ClientError {}

impl From<IOError> for ClientError {
    fn from(error: IOError) -> Self {
        ClientError::IOError(Box::new(error))
    }
}

impl From<RequestError> for ClientError {
    fn from(error: RequestError) -> Self {
        ClientError::RequestError(Box::new(error))
    }
}

impl From<JSONError> for ClientError {
    fn from(error: JSONError) -> Self {
        ClientError::JSONError(Box::new(error))
    }
}

#[cfg(test)]
mod tests_client_error {
    use super::*;

    use rstest::*;
    use std::io::ErrorKind;
    use ureq::Response;

    #[rstest]
    fn test_client_error_display_io_error() {
        // Arrange
        let input = IOError::new(ErrorKind::BrokenPipe, "test error");
        let expected = format!("MBTA Client Error, IO error: {}", input);

        // Act
        let error = ClientError::from(input);
        let actual = format!("{}", error);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_client_error_display_request_error() {
        // Arrange
        let input = RequestError::Status(404, Response::new(404, "Page not found", "foobar").unwrap());
        let expected = format!("MBTA Client Error, request error: {}", input);

        // Act
        let error = ClientError::from(input);
        let actual = format!("{}", error);

        // Assert
        assert_eq!(actual, expected);
    }
}
