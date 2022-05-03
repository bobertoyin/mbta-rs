//! The client for interacting with the V3 API.

use serde::de::DeserializeOwned;

use super::{AlertAttributes, ClientError, FacilityAttributes, Resource, Response};
/// Base url for client request endpoints.
pub const BASE_URL: &str = "https://api-v3.mbta.com";

/// Synchronous client for interacting with the MBTA V3 API.
#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    /// HTTP agent that does all the heavy lifting.
    api_key: Option<String>,
    /// API base URL.
    base_url: String,
}

impl Client {
    /// Create a [Client] without an API key.
    ///
    /// "Without an api key in the query string or as a request header, requests will be tracked by IP address and have stricter rate limit."
    pub fn without_key() -> Self {
        Self {
            api_key: None,
            base_url: BASE_URL.into(),
        }
    }

    /// Create a [Client] with an API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - the API key to use
    pub fn with_key<S: Into<String>>(api_key: S) -> Self {
        Self {
            api_key: Some(api_key.into()),
            base_url: BASE_URL.into(),
        }
    }

    /// Create a [Client] with a custom base URL and no API key.
    /// This method should only be used for mocking/testing purposes.
    ///
    /// # Arguments
    ///
    /// * `base_url` - the base URL to use
    pub fn with_url<S: Into<String>>(base_url: S) -> Self {
        Self {
            api_key: None,
            base_url: base_url.into(),
        }
    }

    /// List all alerts in the MBTA system.
    ///
    /// # Arguments
    ///
    /// * `page_limit` - max number of results per page
    /// * `page_offset` - zero-based number of results to offset by
    pub fn alerts(
        &self,
        page_limit: Option<u64>,
        page_offset: Option<u64>,
    ) -> Result<Response<Vec<Resource<AlertAttributes>>>, ClientError> {
        self.get("alerts", page_limit, page_offset)
    }

    /// Show a particular alert given its ID.
    ///
    /// # Arguments
    ///
    /// * `alert_id` - the ID of the alert
    /// * `query_params` - query parameters for this request
    pub fn alert(
        &self,
        alert_id: &str,
    ) -> Result<Response<Resource<AlertAttributes>>, ClientError> {
        self.get(&format!("alerts/{}", alert_id), None, None)
    }

    /// List all facilities (station amenities) in the MBTA system.
    ///
    /// # Arguments
    ///
    /// * `page_limit` - max number of results per page
    /// * `page_offset` - zero-based number of results to offset by
    pub fn facilities(
        &self,
        page_limit: Option<u64>,
        page_offset: Option<u64>,
    ) -> Result<Response<Vec<Resource<FacilityAttributes>>>, ClientError> {
        self.get("facilities", page_limit, page_offset)
    }

    /// Show a particular facility given its ID.
    ///
    /// # Arguments
    ///
    /// * `facility_id` - the ID of the facility
    pub fn facility(
        &self,
        facility_id: &str,
    ) -> Result<Response<Resource<FacilityAttributes>>, ClientError> {
        self.get(&format!("facility/{}", facility_id), None, None)
    }

    /// List all lines in the MBTA system.
    ///
    /// # Arguments
    ///
    /// * `page_limit` - max number of results per page
    /// * `page_offset` - zero-based number of results to offset by
    pub fn lines(
        &self,
        page_limit: Option<u64>,
        page_offset: Option<u64>,
    ) -> Result<Response<Vec<Resource<FacilityAttributes>>>, ClientError> {
        self.get("lines", page_limit, page_offset)
    }

    // Show a particular line given its ID.
    ///
    /// # Arguments
    ///
    /// * `facility_id` - the ID of the line
    pub fn line(
        &self,
        line_id: &str,
    ) -> Result<Response<Vec<Resource<FacilityAttributes>>>, ClientError> {
        self.get(&format!("lines/{}", line_id), None, None)
    }

    /// Helper method for making generalized GET requests to any endpoint with any query parameters.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - the HTTP endpoint to make a request to
    /// * `page_limit` - max number of results per page
    /// * `page_offset` - zero-based number of results to offset by
    fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        page_limit: Option<u64>,
        page_offset: Option<u64>,
    ) -> Result<T, ClientError> {
        let path = format!("{}/{}", self.base_url, endpoint);
        let request = ureq::get(&path);
        let request = match &self.api_key {
            Some(key) => request.set("x-api-key", key),
            None => request,
        };
        let request = match page_limit {
            Some(limit) => request.query("page[limit]", &limit.to_string()),
            None => request,
        };
        let request = match page_offset {
            Some(offset) => request.query("page[offset]", &offset.to_string()),
            None => request,
        };
        request.call()?.into_json().map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests_client {
    use super::*;

    use httpmock::prelude::*;
    use rstest::*;
    use serde_json::*;
    use std::{fs::File, io::Read, path::PathBuf};

    fn load_json_test_file_contents(relative_file_name: &str) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("resources/test/{}", relative_file_name));
        let mut file = File::open(path).expect(relative_file_name);
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(relative_file_name);
        contents
    }

    #[rstest]
    fn test_client_without_key() {
        // Arrange
        let expected = Client {
            api_key: None,
            base_url: "https://api-v3.mbta.com".into(),
        };

        // Act
        let actual = Client::without_key();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_client_with_key() {
        // Arrange
        let expected = Client {
            api_key: Some("test key".into()),
            base_url: "https://api-v3.mbta.com".into(),
        };

        // Act
        let actual = Client::with_key("test key");

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_client_with_url() {
        // Arrange
        let expected = Client {
            api_key: None,
            base_url: "https://foobar.com".into(),
        };

        // Act
        let actual = Client::with_url("https://foobar.com");

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::valid_response("alerts.json")]
    #[should_panic]
    #[case::invalid_response("bad_request.json")]
    fn tests_client_alerts(#[case] file_path: &str) {
        // Arrange
        let response_body = load_json_test_file_contents(file_path);
        let mock_server = MockServer::start();
        let mock_endpoint = mock_server.mock(|when, then| {
            when.method(GET).path("/alerts");
            then.status(200).body(&response_body);
        });
        let client = Client::with_url(mock_server.base_url());
        let expected: Response<Vec<Resource<AlertAttributes>>> =
            from_str(&response_body).expect("failed to parse");

        // Act
        let actual = client.alerts(None, None).unwrap();

        // Assert
        mock_endpoint.assert();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::valid_response("alert.json")]
    #[should_panic]
    #[case::invalid_response("bad_request.json")]
    fn tests_client_alert(#[case] file_path: &str) {
        // Arrange
        let response_body = load_json_test_file_contents(file_path);
        let mock_server = MockServer::start();
        let mock_endpoint = mock_server.mock(|when, then| {
            when.method(GET).path("/alerts/foobar");
            then.status(200).body(&response_body);
        });
        let client = Client::with_url(mock_server.base_url());
        let expected: Response<Resource<AlertAttributes>> =
            from_str(&response_body).expect("failed to parse");

        // Act
        let actual = client.alert("foobar").unwrap();

        // Assert
        mock_endpoint.assert();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::valid_response("facilities.json")]
    #[should_panic]
    #[case::invalid_response("bad_request.json")]
    fn tests_client_facilities(#[case] file_path: &str) {
        // Arrange
        let response_body = load_json_test_file_contents(file_path);
        let mock_server = MockServer::start();
        let mock_endpoint = mock_server.mock(|when, then| {
            when.method(GET).path("/facilities");
            then.status(200).body(&response_body);
        });
        let client = Client::with_url(mock_server.base_url());
        let expected: Response<Vec<Resource<FacilityAttributes>>> =
            from_str(&response_body).expect("failed to parse");

        // Act
        let actual = client.facilities(None, None).unwrap();

        // Assert
        mock_endpoint.assert();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::valid_response("facility.json")]
    #[should_panic]
    #[case::invalid_response("bad_request.json")]
    fn tests_client_facility(#[case] file_path: &str) {
        // Arrange
        let response_body = load_json_test_file_contents(file_path);
        let mock_server = MockServer::start();
        let mock_endpoint = mock_server.mock(|when, then| {
            when.method(GET).path("/facilities/foobar");
            then.status(200).body(&response_body);
        });
        let client = Client::with_url(mock_server.base_url());
        let expected: Response<Resource<FacilityAttributes>> =
            from_str(&response_body).expect("failed to parse");

        // Act
        let actual = client.facility("foobar").unwrap();

        // Assert
        mock_endpoint.assert();
        assert_eq!(actual, expected);
    }
}
