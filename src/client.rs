//! The client for interacting with the V3 API.

use serde::de::DeserializeOwned;

use super::*;

/// Base url for client request endpoints.
pub const BASE_URL: &str = "https://api-v3.mbta.com";

/// Attribute macro for quickly implementing MBTA client endpoints with multiple return objects.
#[macro_export]
macro_rules! mbta_endpoint_multiple {
    (model=$return_type:ident, func=$endpoint_fn:ident) => {
        impl Client {
            #[doc = "Returns a [Vec] of"]
            #[doc = stringify!($endpoint_fn)]
            #[doc = "in the MBTA system."]
            /// # Arguments
            /// * `page_limit` - max number of results per page
            /// * `page_offset` - zero-based number of results to offset by
            pub fn $endpoint_fn(
                &self,
                page_limit: Option<u64>,
                page_offset: Option<u64>,
            ) -> Result<Response<Vec<Resource<$return_type>>>, ClientError> {
                self.get(stringify!($endpoint_fn), page_limit, page_offset)
            }
        }
    };
}

/// Attribute macro for quickly implementing MBTA client endpoints with single return objects.
#[macro_export]
macro_rules! mbta_endpoint_single {
    (model=$return_type:ident, func=$endpoint_fn:ident, endpoint=$endpoint:expr) => {
        impl Client {
            #[doc = "Returns a"]
            #[doc = stringify!($endpoint_fn)]
            #[doc = "in the MBTA system given its id."]
            /// # Arguments
            #[doc = "* `id` - the id of the"]
            #[doc = stringify!($endpoint_fn)]
            #[doc = "to return"]
            pub fn $endpoint_fn(
                &self,
                id: &str,
            ) -> Result<Response<Resource<$return_type>>, ClientError> {
                self.get(&format!("{}/{}", $endpoint, id), None, None)
            }
        }
    };
}

mbta_endpoint_multiple!(model = AlertAttributes, func = alerts);
mbta_endpoint_multiple!(model = FacilityAttributes, func = facilities);
mbta_endpoint_multiple!(model = LineAttributes, func = lines);
mbta_endpoint_multiple!(model = RouteAttributes, func = routes);
mbta_endpoint_multiple!(model = RoutePatternAttributes, func = route_patterns);

mbta_endpoint_single!(model = AlertAttributes, func = alert, endpoint = "alerts");
mbta_endpoint_single!(
    model = FacilityAttributes,
    func = facility,
    endpoint = "facilities"
);
mbta_endpoint_single!(model = LineAttributes, func = line, endpoint = "lines");
mbta_endpoint_single!(model = RouteAttributes, func = route, endpoint = "routes");
mbta_endpoint_single!(
    model = RoutePatternAttributes,
    func = route_pattern,
    endpoint = "route_patterns"
);

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

    #[macro_export]
    macro_rules! test_from_json_multiple {
        (model=$return_type:ty, test_name=$test_name:ident, method=$method:ident) => {
            #[rstest]
            #[case::valid_response(concat!(stringify!($method), ".json"))]
            #[should_panic]
            #[case::invalid_response("bad_request.json")]
            fn $test_name(#[case] file_path: &str) {
                // Arrange
                let response_body = load_json_test_file_contents(file_path);
                let mock_server = MockServer::start();
                let mock_endpoint = mock_server.mock(|when, then| {
                    when.method(GET).path(concat!("/", stringify!($method)));
                    then.status(200).body(&response_body);
                });
                let client = Client::with_url(mock_server.base_url());
                let expected: Response<Vec<Resource<$return_type>>> =
                    from_str(&response_body).expect("failed to parse");

                // Act
                let actual = client.$method(None, None).unwrap();

                // Assert
                mock_endpoint.assert();
                assert_eq!(actual, expected);
            }
        };
    }

    #[macro_export]
    macro_rules! test_from_json_single {
        (model=$return_type:ty, test_name=$test_name:ident, method=$method:ident, endpoint=$endpoint:expr) => {
            #[rstest]
            #[case::valid_response(concat!(stringify!($method), ".json"))]
            #[should_panic]
            #[case::invalid_response("bad_request.json")]
            fn $test_name(#[case] file_path: &str) {
                // Arrange
                let response_body = load_json_test_file_contents(file_path);
                let mock_server = MockServer::start();
                let mock_endpoint = mock_server.mock(|when, then| {
                    when.method(GET).path(concat!("/", $endpoint, "/foobar"));
                    then.status(200).body(&response_body);
                });
                let client = Client::with_url(mock_server.base_url());
                let expected: Response<Resource<$return_type>> =
                    from_str(&response_body).expect("failed to parse");

                // Act
                let actual = client.$method("foobar").unwrap();

                // Assert
                mock_endpoint.assert();
                assert_eq!(actual, expected);
            }
        };
    }

    test_from_json_multiple!(
        model = AlertAttributes,
        test_name = test_client_alerts,
        method = alerts
    );
    test_from_json_multiple!(
        model = FacilityAttributes,
        test_name = test_client_facilities,
        method = facilities
    );
    test_from_json_multiple!(
        model = LineAttributes,
        test_name = test_client_lines,
        method = lines
    );
    test_from_json_multiple!(
        model = RouteAttributes,
        test_name = test_client_routes,
        method = routes
    );
    test_from_json_multiple!(
        model = RoutePatternAttributes,
        test_name = test_client_route_patterns,
        method = route_patterns
    );

    test_from_json_single!(
        model = AlertAttributes,
        test_name = test_client_alert,
        method = alert,
        endpoint = "alerts"
    );
    test_from_json_single!(
        model = FacilityAttributes,
        test_name = test_client_facility,
        method = facility,
        endpoint = "facilities"
    );
    test_from_json_single!(
        model = LineAttributes,
        test_name = test_client_line,
        method = line,
        endpoint = "lines"
    );
    test_from_json_single!(
        model = RouteAttributes,
        test_name = test_client_route,
        method = route,
        endpoint = "routes"
    );
    test_from_json_single!(
        model = RoutePatternAttributes,
        test_name = test_client_route_pattern,
        method = route_pattern,
        endpoint = "route_patterns"
    );

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
}
