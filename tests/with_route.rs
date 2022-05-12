//! Sanity testing against the endpoints that need a route filter at bare minimum.

/// Macro for creating sanity tests for endpoints with plural return values.
/// These endpoints also need a route filter.
#[macro_export]
macro_rules! test_endpoint_plural_with_route {
    (plural_func=$plural_func:ident) => {
        #[cfg(test)]
        mod $plural_func {
            use std::collections::HashMap;

            use rstest::*;

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
                let routes = client.routes(HashMap::from([("page[limit]".into(), "1".into())])).expect("failed to get routes");

                // Act
                let $plural_func = client
                    .$plural_func(HashMap::from([
                        ("page[limit]".into(), "3".into()),
                        ("filter[route]".into(), routes.data[0].id.clone()),
                    ]))
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
                if let ClientError::ResponseError { errors } = error {
                    assert_eq!(errors.errors.len(), 1);
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
                if let ClientError::InvalidQueryParam { name, value } = error {
                    assert_eq!(name, "foo");
                    assert_eq!(value, "bar");
                } else {
                    panic!("wrong error type");
                }
            }
        }
    };
}

/// Macro for creating sanity tests for endpoints with singular return values.
/// These endpoints also need a route filter.
#[macro_export]
macro_rules! test_endpoint_singular_with_route {
    (plural_func=$plural_func:ident, singular_func=$singular_func:ident) => {
        #[cfg(test)]
        mod $singular_func {
            use std::collections::HashMap;

            use rstest::*;

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
            fn success_singular_model(client: Client) {
                // Arrange
                let routes = client.routes(HashMap::from([("page[limit]".into(), "1".into())])).expect("failed to get routes");
                let $plural_func = client
                    .$plural_func(HashMap::from([
                        ("page[limit]".into(), "3".into()),
                        ("filter[route]".into(), routes.data[0].id.clone()),
                    ]))
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
                let error = client.$singular_func("foobar").expect_err(&format!("{} did not fail", stringify!($singular_func)));

                // Assert
                if let ClientError::ResponseError { errors } = error {
                    assert_eq!(errors.errors.len(), 1);
                } else {
                    panic!("wrong error type");
                }
            }
        }
    };
}

test_endpoint_plural_with_route!(plural_func = predictions);
test_endpoint_plural_with_route!(plural_func = services);
test_endpoint_plural_with_route!(plural_func = schedules);
test_endpoint_plural_with_route!(plural_func = shapes);
test_endpoint_plural_with_route!(plural_func = trips);

test_endpoint_singular_with_route!(plural_func = services, singular_func = service);
test_endpoint_singular_with_route!(plural_func = shapes, singular_func = shape);
test_endpoint_singular_with_route!(plural_func = trips, singular_func = trip);
