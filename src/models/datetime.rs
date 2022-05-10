//! Serialization and deserialization methods for MBTA dates and datetimes.

/// Datetime string format.
pub const DATETIME_FORMAT: &str = "%FT%T%:z";

/// Date string format.
pub const DATE_FORMAT: &str = "%F";

/// Serialization and deserialization for the MBTA datetime format.
pub mod mbta_datetime_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::DATETIME_FORMAT;

    /// Serialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `datetime` - the datetime
    /// * `serializer` - the serializer
    pub fn serialize<S: Serializer>(datetime: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", datetime.format(DATETIME_FORMAT)))
    }

    /// Attempt to deserialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error> {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, DATETIME_FORMAT).map_err(serde::de::Error::custom)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rstest::*;
        use serde_json::{Deserializer, Serializer};

        #[fixture]
        fn serializer() -> Serializer<Vec<u8>> {
            Serializer::new(Vec::new())
        }

        #[rstest]
        #[case::simple_case(
            DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input"), 
            "\"2022-05-08T13:18:08-04:00\"",
        )]
        fn test_serialize(mut serializer: Serializer<Vec<u8>>, #[case] input: DateTime<FixedOffset>, #[case] expected: &str) {
            // Arrange

            // Act
            serialize(&input, &mut serializer).expect("failed to serialize");
            let inner = serializer.into_inner();
            let actual = std::str::from_utf8(&inner).expect("failed to convert to string");

            // Assert
            assert_eq!(actual, expected);
        }

        #[rstest]
        #[case::valid_format(
            "\"2022-05-08T13:18:08-04:00\"",
            DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input"),
        )]
        #[should_panic = "failed to deserialize"]
        #[case::invalid_format(
            "\"2022-05-08 13:18:08-04:00\"",
            DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input"),
        )]
        fn test_deserialize(#[case] input: &str, #[case] expected: DateTime<FixedOffset>) {
            // Arrange
            let mut deserializer = Deserializer::from_str(input);

            // Act
            let actual = deserialize(&mut deserializer).expect("failed to deserialize");

            // Assert
            assert_eq!(actual, expected);
        }
    }
}

/// Serialization and deserialization for an optional MBTA datetime format.
pub mod optional_mbta_datetime_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::{mbta_datetime_format::serialize as datetime_serialize, DATETIME_FORMAT};

    /// Serialize an optional MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `datetime` - the optional datetime
    /// * `serializer` - the serializer
    pub fn serialize<S>(datetime: &Option<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(d) => datetime_serialize(d, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Attempt to deserialize an optional MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let date = DateTime::parse_from_str(&s, DATETIME_FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rstest::*;
        use serde_json::{Deserializer, Serializer};

        #[fixture]
        fn serializer() -> Serializer<Vec<u8>> {
            Serializer::new(Vec::new())
        }

        #[rstest]
        #[case::some_dateime(
            Some(DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input")), 
            "\"2022-05-08T13:18:08-04:00\"",
        )]
        #[case::no_datetime(None, "null")]
        fn test_serialize(mut serializer: Serializer<Vec<u8>>, #[case] input: Option<DateTime<FixedOffset>>, #[case] expected: &str) {
            // Arrange

            // Act
            serialize(&input, &mut serializer).expect("failed to serialize");
            let inner = serializer.into_inner();
            let actual = std::str::from_utf8(&inner).expect("failed to convert to string");

            // Assert
            assert_eq!(actual, expected);
        }

        #[rstest]
        #[case::valid_format(
            "\"2022-05-08T13:18:08-04:00\"",
            Some(DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input")), 
        )]
        #[case::valid_format("null", None)]
        #[should_panic = "failed to deserialize"]
        #[case::invalid_format("\"2022-05-08 13:18:08-04:00\"", None)]
        fn test_deserialize(#[case] input: &str, #[case] expected: Option<DateTime<FixedOffset>>) {
            // Arrange
            let mut deserializer = Deserializer::from_str(input);

            // Act
            let actual = deserialize(&mut deserializer).expect("failed to deserialize");

            // Assert
            assert_eq!(actual, expected);
        }
    }
}

/// Serialization and deserialization for the MBTA date format.
pub mod mbta_date_format {
    use chrono::{Date, DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::{DATETIME_FORMAT, DATE_FORMAT};

    /// Serialize an MBTA date.
    ///
    /// # Arguments
    ///
    /// * `date` - the date
    /// * `serializer` - the serializer
    pub fn serialize<S: Serializer>(date: &Date<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", date.format(DATE_FORMAT)))
    }

    /// Attempt to deserialize an MBTA date.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Date<FixedOffset>, D::Error> {
        let s = format!("{}T00:00:00-04:00", String::deserialize(deserializer)?);
        DateTime::parse_from_str(&s, DATETIME_FORMAT).map(|dt| dt.date()).map_err(serde::de::Error::custom)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use chrono::{Date, DateTime, FixedOffset};
        use rstest::*;
        use serde_json::{Deserializer, Serializer};

        #[fixture]
        fn serializer() -> Serializer<Vec<u8>> {
            Serializer::new(Vec::new())
        }

        #[rstest]
        #[case::simple_case(
            DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date(), 
            "\"2022-05-08\""
        )]
        fn test_serialize(mut serializer: Serializer<Vec<u8>>, #[case] input: Date<FixedOffset>, #[case] expected: &str) {
            // Arrange

            // Act
            serialize(&input, &mut serializer).expect("failed to serialize");
            let inner = serializer.into_inner();
            let actual = std::str::from_utf8(&inner).expect("failed to convert to string");

            // Assert
            assert_eq!(actual, expected);
        }

        #[rstest]
        #[case::valid_format(
            "\"2022-05-08\"",
            DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date(), 
        )]
        #[should_panic = "failed to deserialize"]
        #[case::invalid_format(
            "\"2022 05 08\"",
            DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date(), 
        )]
        fn test_deserialize(#[case] input: &str, #[case] expected: Date<FixedOffset>) {
            // Arrange
            let mut deserializer = Deserializer::from_str(input);

            // Act
            let actual = deserialize(&mut deserializer).expect("failed to deserialize");

            // Assert
            assert_eq!(actual, expected);
        }
    }
}

/// Serialization and deserialization for an optional MBTA date format.
pub mod optional_mbta_date_format {
    use chrono::{Date, DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::{mbta_date_format::serialize as date_serialize, DATETIME_FORMAT};

    /// Serialize an optional MBTA date.
    ///
    /// # Arguments
    ///
    /// * `date` - the optional date
    /// * `serializer` - the serializer
    pub fn serialize<S>(date: &Option<Date<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => date_serialize(d, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Attempt to deserialize an optional MBTA date.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let date = DateTime::parse_from_str(&format!("{}T00:00:00-04:00", s), DATETIME_FORMAT)
                    .map(|dt| dt.date())
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use chrono::{Date, DateTime, FixedOffset};
        use rstest::*;
        use serde_json::{Deserializer, Serializer};

        #[fixture]
        fn serializer() -> Serializer<Vec<u8>> {
            Serializer::new(Vec::new())
        }

        #[rstest]
        #[case::some_date(
            Some(DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date()), 
            "\"2022-05-08\"",
        )]
        #[case::no_date(None, "null")]
        fn test_serialize(mut serializer: Serializer<Vec<u8>>, #[case] input: Option<Date<FixedOffset>>, #[case] expected: &str) {
            // Arrange

            // Act
            serialize(&input, &mut serializer).expect("failed to serialize");
            let inner = serializer.into_inner();
            let actual = std::str::from_utf8(&inner).expect("failed to convert to string");

            // Assert
            assert_eq!(actual, expected);
        }

        #[rstest]
        #[case::valid_format(
            "\"2022-05-08\"",
            Some(DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date()), 
        )]
        #[case::no_date("null", None)]
        #[should_panic = "failed to deserialize"]
        #[case::invalid_format("\"2022 05 08\"", None)]
        fn test_deserialize(#[case] input: &str, #[case] expected: Option<Date<FixedOffset>>) {
            // Arrange
            let mut deserializer = Deserializer::from_str(input);

            // Act
            let actual = deserialize(&mut deserializer).expect("failed to deserialize");

            // Assert
            assert_eq!(actual, expected);
        }
    }
}

/// Serialization and deserialization for an vector of MBTA dates format.
pub mod vec_mbta_date_format {
    use chrono::{Date, DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::{DATETIME_FORMAT, DATE_FORMAT};

    /// Serialize a vector of MBTA dates.
    ///
    /// # Arguments
    ///
    /// * `dates` - the dates
    /// * `serializer` - the serializer
    pub fn serialize<S>(dates: &[Date<FixedOffset>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(dates.iter().map(|dt| format!("{}", dt.format(DATE_FORMAT))))
    }

    /// Attempt to deserialize an optional MBTA dates.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Date<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::<String>::deserialize(deserializer)?;
        let mut dates = Vec::new();
        for dt in v {
            dates.push(
                DateTime::parse_from_str(&format!("{}T00:00:00-04:00", dt), DATETIME_FORMAT)
                    .map(|dt| dt.date())
                    .map_err(serde::de::Error::custom)?,
            )
        }
        Ok(dates)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use chrono::{Date, DateTime, FixedOffset};
        use rstest::*;
        use serde_json::{Deserializer, Serializer};

        #[fixture]
        fn serializer() -> Serializer<Vec<u8>> {
            Serializer::new(Vec::new())
        }

        #[rstest]
        #[case::some_dates(
            vec![DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date()], 
            "[\"2022-05-08\"]",
        )]
        #[case::no_dates(vec![], "[]")]
        fn test_serialize(mut serializer: Serializer<Vec<u8>>, #[case] input: Vec<Date<FixedOffset>>, #[case] expected: &str) {
            // Arrange

            // Act
            serialize(&input, &mut serializer).expect("failed to serialize");
            let inner = serializer.into_inner();
            let actual = std::str::from_utf8(&inner).expect("failed to convert to string");

            // Assert
            assert_eq!(actual, expected);
        }

        #[rstest]
        #[case::valid_format(
            "[\"2022-05-08\"]",
            vec![DateTime::parse_from_str("2022-05-08T13:18:08-04:00", "%FT%T%:z").expect("invalid input").date()], 
        )]
        #[case::no_dates("[]", vec![])]
        #[should_panic = "failed to deserialize"]
        #[case::invalid_format("[\"2022 05 08\"]", vec![])]
        fn test_deserialize(#[case] input: &str, #[case] expected: Vec<Date<FixedOffset>>) {
            // Arrange
            let mut deserializer = Deserializer::from_str(input);

            // Act
            let actual = deserialize(&mut deserializer).expect("failed to deserialize");

            // Assert
            assert_eq!(actual, expected);
        }
    }
}
