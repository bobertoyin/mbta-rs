//! Sanity testing against simple endpoints: endpoints with both a plural and singular form and don't require any filter query parameters.

/// Macro for creating sanity tests for endpoints with plural return values and single return values.
#[macro_export]
macro_rules! test_endpoint_plural_and_singular {
    (plural_func=$plural_func:ident, singular_func=$singular_func:ident) => {
        #[cfg(test)]
        mod $plural_func {
            use std::collections::HashMap;

            use rstest::*;
            use ureq::Error;

            use mbta_rs::*;

            #[fixture]
            fn client() -> Client {
                if let Ok(token) = std::env::var("MBTA_TOKEN") {
                    Client::with_key(token)
                } else {
                    Client::without_key()
                }
            }

            #[rstest]
            fn success_plural_models(client: Client) {
                // Arrange

                // Act
                let $plural_func = client
                    .$plural_func(HashMap::from([("page[limit]".into(), "3".into())]))
                    .expect(&format!("failed to get {}", stringify!($plural_func)));

                // Assert
                assert_eq!($plural_func.data.len(), 3);
                assert_eq!($plural_func.jsonapi, APIVersion { version: "1.0".into() });
                assert!($plural_func.links.is_some());
            }

            #[rstest]
            fn request_failure_plural_models(client: Client) {
                // Arrange

                // Act
                let error = client
                    .$plural_func(HashMap::from([("sort".into(), "foobar".into())]))
                    .expect_err(&format!("{} did not fail", stringify!($plural_func)));

                // Assert
                if let ClientError::RequestError(e) = error {
                    if let Error::Status(code, response) = *e {
                        assert_eq!(code, 400);
                        assert_eq!(response.status(), 400);
                        assert_eq!(response.status_text(), "Bad Request");
                        assert_eq!(response.get_url(), format!("https://api-v3.mbta.com/{}?sort=foobar", stringify!($plural_func)));
                    } else {
                        panic!("wrong request error type");
                    }
                } else {
                    panic!("wrong error type");
                }
            }

            #[rstest]
            fn query_param_failure_plural_models(client: Client) {
                // Arrange

                // Act
                let error = client
                    .$plural_func(HashMap::from([("foo".into(), "bar".into())]))
                    .expect_err(&format!("{} did not fail", stringify!($plural_func)));

                // Assert
                if let ClientError::InvalidQueryParam(k, v) = error {
                    assert_eq!(k, "foo");
                    assert_eq!(v, "bar");
                } else {
                    panic!("wrong error type");
                }
            }

            #[rstest]
            fn success_singular_model(client: Client) {
                // Arrange
                let $plural_func = client
                    .$plural_func(HashMap::from([("page[limit]".into(), "3".into())]))
                    .expect(&format!("failed to get {}", stringify!($plural_func)));

                // Act & Assert
                for $singular_func in $plural_func.data {
                    let actual = client
                        .$singular_func(&$singular_func.id)
                        .expect(&format!("failed to get {}", stringify!($singular_func)));
                    assert_eq!(actual.data.id, $singular_func.id);
                    assert_eq!(actual.jsonapi, APIVersion { version: "1.0".into() });
                    assert!(actual.links.is_none());
                }
            }

            #[rstest]
            fn request_failure_singular_model(client: Client) {
                // Arrange

                // Act
                let error = client.$singular_func("foobar").expect_err("facility did not fail");

                // Assert
                if let ClientError::RequestError(e) = error {
                    if let Error::Status(code, response) = *e {
                        assert_eq!(code, 404);
                        assert_eq!(response.status(), 404);
                        assert_eq!(response.status_text(), "Not Found");
                        assert_eq!(response.get_url(), format!("https://api-v3.mbta.com/{}/foobar", stringify!($plural_func)));
                    } else {
                        panic!("wrong request error type");
                    }
                } else {
                    panic!("wrong error type");
                }
            }
        }
    };
}

test_endpoint_plural_and_singular!(plural_func = alerts, singular_func = alert);
test_endpoint_plural_and_singular!(plural_func = facilities, singular_func = facility);
test_endpoint_plural_and_singular!(plural_func = lines, singular_func = line);
test_endpoint_plural_and_singular!(plural_func = route_patterns, singular_func = route_pattern);
test_endpoint_plural_and_singular!(plural_func = routes, singular_func = route);
test_endpoint_plural_and_singular!(plural_func = stops, singular_func = stop);
test_endpoint_plural_and_singular!(plural_func = vehicles, singular_func = vehicle);
