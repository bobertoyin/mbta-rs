//! The client for interacting with the V3 API.

use std::collections::HashSet;

use serde::de::DeserializeOwned;

use super::*;

/// Base url for client request endpoints.
pub const BASE_URL: &str = "https://api-v3.mbta.com";

/// Attribute macro for quickly implementing MBTA client endpoints with multiple return objects.
#[doc(hidden)]
#[macro_export]
macro_rules! mbta_endpoint_multiple {
    (model=$model:ident, func=$func:ident, allowed_query_params=$allowed_query_params:expr) => {
        impl Client {
            #[doc = concat!("Returns ", stringify!($func), " in the MBTA system.")]
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
            /// * `query_params` - a slice of pairings of query parameter names to values
            ///
            /// ```
            /// # use std::env;
            /// # use mbta_rs::Client;
            /// #
            /// # let client = match env::var("MBTA_TOKEN") {
            /// #     Ok(token) => Client::with_key(token),
            /// #     Err(_) => Client::without_key()
            /// # };
            /// #
            /// # let query_params = [
            /// #     ("page[limit]", "3")
            /// # ];
            #[doc = concat!("let ", stringify!($func), "_response = client.", stringify!($func), "(&query_params);\n")]
            #[doc = concat!("if let Ok(", stringify!($func), ") = ", stringify!($func), "_response {\n")]
            #[doc = concat!("    for item in ", stringify!($func), ".data {\n")]
            ///         println!("{}", item.id);
            ///     }
            /// }
            /// ```
            pub fn $func<K: AsRef<str>, V: AsRef<str>>(&self, query_params: &[(K, V)]) -> Result<Response<$model>, ClientError> {
                let allowed_query_params: HashSet<String> = $allowed_query_params.into_iter().map(|s: &str| s.to_string()).collect();
                for (k, v) in query_params {
                    if !allowed_query_params.contains(k.as_ref()) {
                        return Err(ClientError::InvalidQueryParam {
                            name: k.as_ref().to_string(),
                            value: v.as_ref().to_string(),
                        });
                    }
                }
                self.get(stringify!($func), query_params)
            }
        }
    };
}

/// Attribute macro for quickly implementing MBTA client endpoints with single return objects.
#[doc(hidden)]
#[macro_export]
macro_rules! mbta_endpoint_single {
    (model=$model:ident, func=$func:ident, endpoint=$endpoint:expr, allowed_query_params=$allowed_query_params:expr) => {
        impl Client {
            #[doc = concat!("Returns a ", stringify!($func), " in the MBTA system given its id.")]
            ///
            /// # Arguments
            #[doc = concat!("* `id` - the id of the ", stringify!($func), " to return")]
            ///
            /// ```
            /// # use std::env;
            /// # use mbta_rs::Client;
            /// #
            /// # let client = match env::var("MBTA_TOKEN") {
            /// #     Ok(token) => Client::with_key(token),
            /// #     Err(_) => Client::without_key()
            /// # };
            /// #
            /// # let id = "";
            #[doc = concat!("let ", stringify!($func), "_response = client.", stringify!($func), "(id);\n")]
            #[doc = concat!("if let Ok(item) = ", stringify!($func), "_response {\n")]
            ///     println!("{}", item.data.id);
            /// }
            /// ```
            pub fn $func(&self, id: &str) -> Result<Response<$model>, ClientError> {
                self.get::<$model, String, String>(&format!("{}/{}", $endpoint, id), &[])
            }
        }
    };
}

mbta_endpoint_multiple!(
    model = Alerts,
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
    model = Facilities,
    func = facilities,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[stop]", "filter[type]",]
);
mbta_endpoint_multiple!(
    model = Lines,
    func = lines,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[id]",]
);
mbta_endpoint_multiple!(
    model = LiveFacilities,
    func = live_facilities,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[id]",]
);
mbta_endpoint_multiple!(
    model = Predictions,
    func = predictions,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[latitude]",
        "filter[longitude]",
        "filter[radius]",
        "filter[direction_id]",
        "filter[route_type]",
        "filter[stop]",
        "filter[route]",
        "filter[trip]",
        "filter[route_pattern]",
    ]
);
mbta_endpoint_multiple!(
    model = Routes,
    func = routes,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[stop]",
        "filter[type]",
        "filter[direction_id]",
        "filter[date]",
        "filter[id]",
    ]
);
mbta_endpoint_multiple!(
    model = RoutePatterns,
    func = route_patterns,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[id]",
        "filter[route]",
        "filter[direction_id]",
        "filter[stop]",
    ]
);
mbta_endpoint_multiple!(
    model = Schedules,
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
mbta_endpoint_multiple!(
    model = Services,
    func = services,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[id]", "filter[route]",]
);
mbta_endpoint_multiple!(
    model = Shapes,
    func = shapes,
    allowed_query_params = ["page[offset]", "page[limit]", "sort", "filter[route]",]
);
mbta_endpoint_multiple!(
    model = Stops,
    func = stops,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[date]",
        "filter[direction_id]",
        "filter[latitude]",
        "filter[longitude]",
        "filter[radius]",
        "filter[id]",
        "filter[route_type]",
        "filter[route]",
        "filter[service]",
        "filter[location_type]",
    ]
);
mbta_endpoint_multiple!(
    model = Trips,
    func = trips,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[date]",
        "filter[direction_id]",
        "filter[route]",
        "filter[route_pattern]",
        "filter[id]",
        "filter[name]",
    ]
);
mbta_endpoint_multiple!(
    model = Vehicles,
    func = vehicles,
    allowed_query_params = [
        "page[offset]",
        "page[limit]",
        "sort",
        "filter[id]",
        "filter[trip]",
        "filter[label]",
        "filter[route]",
        "filter[direction_id]",
        "filter[route_type]",
    ]
);

mbta_endpoint_single!(model = Alert, func = alert, endpoint = "alerts", allowed_query_params = []);
mbta_endpoint_single!(model = Facility, func = facility, endpoint = "facilities", allowed_query_params = []);
mbta_endpoint_single!(model = Line, func = line, endpoint = "lines", allowed_query_params = []);
mbta_endpoint_single!(model = Route, func = route, endpoint = "routes", allowed_query_params = []);
mbta_endpoint_single!(model = RoutePattern, func = route_pattern, endpoint = "route_patterns", allowed_query_params = []);
mbta_endpoint_single!(model = Service, func = service, endpoint = "services", allowed_query_params = []);
mbta_endpoint_single!(model = Shape, func = shape, endpoint = "shapes", allowed_query_params = []);
mbta_endpoint_single!(model = Stop, func = stop, endpoint = "stops", allowed_query_params = []);
mbta_endpoint_single!(model = Trip, func = trip, endpoint = "trips", allowed_query_params = []);
mbta_endpoint_single!(model = Vehicle, func = vehicle, endpoint = "vehicles", allowed_query_params = []);

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
    /// > "Without an api key in the query string or as a request header, requests will be tracked by IP address and have stricter rate limit." - Massachusetts Bay Transportation Authority
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

    /// Helper method for making generalized `GET` requests to any endpoint with any query parameters.
    /// Presumes that all query parameters given are valid.
    ///
    /// # Arguments
    ///
    /// * query_params - a slice of pairings of query parameter names to values
    fn get<T: DeserializeOwned, K: AsRef<str>, V: AsRef<str>>(
        &self,
        endpoint: &str,
        query_params: &[(K, V)],
    ) -> Result<Response<T>, ClientError> {
        let path = format!("{}/{}", self.base_url, endpoint);
        let request = ureq::get(&path);
        let request = match &self.api_key {
            Some(key) => request.set("x-api-key", key),
            None => request,
        };
        let request = query_params.iter().fold(request, |r, (k, v)| r.query(k.as_ref(), v.as_ref()));
        let response: Response<T> = request.call()?.into_json()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

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
