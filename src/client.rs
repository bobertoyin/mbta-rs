//! The client for interacting with the V3 API.

use std::collections::{HashMap, HashSet};

use serde::de::DeserializeOwned;

use serde_json::{from_value, Error as JSONError, Value};

use super::*;

/// Base url for client request endpoints.
pub const BASE_URL: &str = "https://api-v3.mbta.com";

/// Attribute macro for quickly implementing MBTA client endpoints with multiple return objects.
#[macro_export]
macro_rules! mbta_endpoint_multiple {
    (model=$model:ident, func=$endpoint_fn:ident, allowed_query_params=$allowed_query_params:expr) => {
        impl Client {
            #[doc = "Returns a [Vec] of"]
            #[doc = stringify!($endpoint_fn)]
            #[doc = "in the MBTA system."]
            ///
            /// Consult the [API swagger docs](https://api-v3.mbta.com/docs/swagger/index.html) for each parameter's meaning and which are required,
            /// but the request will fail if you include any that are *not* the ones specified below
            /// (we limit them to avoid any return type behaviors that we currently can't support).
            ///
            /// # Allowed Query Parameters
            ///
            #[doc = concat!("`", stringify!($allowed_query_params), "`")]
            ///
            /// # Arguments
            ///
            /// * `query_params` - a [HashMap] of query parameter names to values
            pub fn $endpoint_fn(&self, query_params: HashMap<String, String>) -> Result<Response<Vec<Resource<$model>>>, ClientError> {
                let allowed_query_params: HashSet<String> = $allowed_query_params.into_iter().map(|s: &str| s.to_string()).collect();
                for (k, v) in &query_params {
                    if !allowed_query_params.contains(&k.to_string()) {
                        return Err(ClientError::InvalidQueryParam(k.to_string(), v.to_string()));
                    }
                }
                self.get(stringify!($endpoint_fn), query_params)
            }
        }
    };
}

/// Attribute macro for quickly implementing MBTA client endpoints with single return objects.
#[macro_export]
macro_rules! mbta_endpoint_single {
    (model=$model:ident, func=$endpoint_fn:ident, endpoint=$endpoint:expr, allowed_query_params=$allowed_query_params:expr) => {
        impl Client {
            #[doc = "Returns a"]
            #[doc = stringify!($endpoint_fn)]
            #[doc = "in the MBTA system given its id."]
            ///
            /// Consult the [API swagger docs](https://api-v3.mbta.com/docs/swagger/index.html) for each parameter's meaning and which are required,
            /// but the request will fail if you include any that are *not* the ones specified below
            /// (we limit them to avoid any return type behaviors that we currently can't support).
            ///
            /// # Allowed Query Parameters
            ///
            #[doc = concat!("`", stringify!($allowed_query_params), "`")]
            ///
            /// # Arguments
            #[doc = "* `id` - the id of the"]
            #[doc = stringify!($endpoint_fn)]
            #[doc = "to return"]
            /// * `query_params` - a [HashMap] of query parameter names to values
            pub fn $endpoint_fn(&self, id: &str, query_params: HashMap<String, String>) -> Result<Response<Resource<$model>>, ClientError> {
                let allowed_query_params: HashSet<String> = $allowed_query_params.into_iter().map(|s: &str| s.to_string()).collect();
                for (k, v) in &query_params {
                    if !allowed_query_params.contains(&k.to_string()) {
                        return Err(ClientError::InvalidQueryParam(k.to_string(), v.to_string()));
                    }
                }
                self.get(&format!("{}/{}", $endpoint, id), query_params)
            }
        }
    };
}

mbta_endpoint_multiple!(
    model = AlertAttributes,
    func = alerts,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[activity]",
        "filter[route_type]",
        "filter[direction_id]",
        "filter[route]",
        "filter[stop]",
        "filter[trip]",
        "filter[facility]",
        "filter[id]",
        "filter[banner]",
        "filter[datetime]",
        "filter[lifecycle]",
        "filter[severity]",
    ]
);
mbta_endpoint_multiple!(
    model = FacilityAttributes,
    func = facilities,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[stop]", "filter[type]",]
);
mbta_endpoint_multiple!(
    model = LineAttributes,
    func = lines,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[id]",]
);
mbta_endpoint_multiple!(
    model = RouteAttributes,
    func = routes,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "include",
        "filter[stop]",
        "filter[type]",
        "filter[direction_id]",
        "filter[date]",
        "filter[id]",
    ]
);
mbta_endpoint_multiple!(
    model = RoutePatternAttributes,
    func = route_patterns,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "include",
        "filter[id]",
        "filter[route]",
        "filter[direction_id]",
        "filter[stop]",
    ]
);
mbta_endpoint_multiple!(
    model = ScheduleAttributes,
    func = schedules,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[date]",
        "filter[direction_id]",
        "filter[route_type]",
        "filter[min_time]",
        "filter[max_time]",
        "filter[route]",
        "filter[stop]",
        "filter[trip]",
        "filter[stop_sequence]",
    ]
);

mbta_endpoint_single!(model = AlertAttributes, func = alert, endpoint = "alerts", allowed_query_params = []);
mbta_endpoint_single!(
    model = FacilityAttributes,
    func = facility,
    endpoint = "facilities",
    allowed_query_params = []
);
mbta_endpoint_single!(model = LineAttributes, func = line, endpoint = "lines", allowed_query_params = []);
mbta_endpoint_single!(model = RouteAttributes, func = route, endpoint = "routes", allowed_query_params = []);
mbta_endpoint_single!(
    model = RoutePatternAttributes,
    func = route_pattern,
    endpoint = "route_patterns",
    allowed_query_params = []
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
    /// Presumes that all query parameters given in the [HashMap] are valid.
    ///
    /// # Arguments
    ///
    /// * query_params - a [HashMap] of query parameter names to values
    fn get<T: DeserializeOwned>(&self, endpoint: &str, query_params: HashMap<String, String>) -> Result<Response<T>, ClientError> {
        let path = format!("{}/{}", self.base_url, endpoint);
        let request = ureq::get(&path);
        let request = match &self.api_key {
            Some(key) => request.set("x-api-key", key),
            None => request,
        };
        let request = query_params.iter().fold(request, |r, (k, v)| r.query(k, v));
        let json: Value = request.call()?.into_json()?;
        let try_success: Result<ResponseSuccess<T>, JSONError> = from_value(json.clone());
        match try_success {
            Ok(result) => Ok(result.into()),
            Err(e) => {
                println!("{:?}", e);
                let try_error: Result<ResponseError, JSONError> = from_value(json);
                try_error.map(|r| r.into()).map_err(|e| e.into())
            }
        }
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
        file.read_to_string(&mut contents).expect(relative_file_name);
        contents
    }

    #[macro_export]
    macro_rules! test_from_json_multiple {
        (model=$model:ty, test_name=$test_name:ident, method=$method:ident) => {
            #[rstest]
            #[case::success_response(concat!(stringify!($method), ".json"), &[("page[limit]", "barfoo")])]
            #[should_panic]
            #[case::invalid_query(concat!(stringify!($method), ".json"), &[("bad", "barfoo")])]
            #[case::error_response("bad_request.json", &[("page[limit]", "barfoo")])]
            fn $test_name(#[case] file_path: &str, #[case] test_queries: &[(&str, &str)]) {
                // Arrange
                let response_body = load_json_test_file_contents(file_path);
                let mock_server = MockServer::start();
                let mock_endpoint = mock_server.mock(|when, then| {
                    when.method(GET).path(concat!("/", stringify!($method))).query_param("page[limit]", "barfoo");
                    then.status(200).body(&response_body);
                });
                let client = Client::with_url(mock_server.base_url());
                let expected: Response<Vec<Resource<$model>>> = from_str(&response_body).expect("failed to parse");
                let mut queries = HashMap::new();
                for (k, v) in test_queries {
                    queries.insert(k.to_string(), v.to_string());
                }

                // Act
                let actual = client.$method(queries).unwrap();

                // Assert
                mock_endpoint.assert();
                assert_eq!(actual, expected);
            }
        };
    }

    #[macro_export]
    macro_rules! test_from_json_single {
        (model=$model:ty, test_name=$test_name:ident, method=$method:ident, endpoint=$endpoint:expr) => {
            #[rstest]
            #[case::success_response(concat!(stringify!($method), ".json"), &[])]
            #[should_panic]
            #[case::invalid_query(concat!(stringify!($method), ".json"), &[("bad", "barfoo")])]
            #[case::error_response("bad_request.json", &[])]
            fn $test_name(#[case] file_path: &str, #[case] test_queries: &[(&str, &str)]) {
                // Arrange
                let response_body = load_json_test_file_contents(file_path);
                let mock_server = MockServer::start();
                let mock_endpoint = mock_server.mock(|when, then| {
                    when.method(GET).path(concat!("/", $endpoint, "/foobar"));
                    then.status(200).body(&response_body);
                });
                let client = Client::with_url(mock_server.base_url());
                let expected: Response<Resource<$model>> = from_str(&response_body).expect("failed to parse");
                let mut queries = HashMap::new();
                for (k, v) in test_queries {
                    queries.insert(k.to_string(), v.to_string());
                }

                // Act
                let actual = client.$method("foobar", queries).unwrap();

                // Assert
                mock_endpoint.assert();
                assert_eq!(actual, expected);
            }
        };
    }

    test_from_json_multiple!(model = AlertAttributes, test_name = test_client_alerts, method = alerts);
    test_from_json_multiple!(model = FacilityAttributes, test_name = test_client_facilities, method = facilities);
    test_from_json_multiple!(model = LineAttributes, test_name = test_client_lines, method = lines);
    test_from_json_multiple!(model = RouteAttributes, test_name = test_client_routes, method = routes);
    test_from_json_multiple!(model = RoutePatternAttributes, test_name = test_client_route_patterns, method = route_patterns);
    test_from_json_multiple!(model = ScheduleAttributes, test_name = test_client_schedules, method = schedules);

    test_from_json_single!(model = AlertAttributes, test_name = test_client_alert, method = alert, endpoint = "alerts");
    test_from_json_single!(
        model = FacilityAttributes,
        test_name = test_client_facility,
        method = facility,
        endpoint = "facilities"
    );
    test_from_json_single!(model = LineAttributes, test_name = test_client_line, method = line, endpoint = "lines");
    test_from_json_single!(model = RouteAttributes, test_name = test_client_route, method = route, endpoint = "routes");
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
