//! Serialization and deserialization methods for MBTA dates and datetimes.

/// Serialization and deserialization for the MBTA datetime format.
pub mod mbta_datetime_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Datetime string format.
    pub const FORMAT: &str = "%FT%T%:z";

    /// Serialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `datetime` - the datetime
    /// * `serializer` - the serializer
    pub fn serialize<S: Serializer>(datetime: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", datetime.format(FORMAT)))
    }

    /// Attempt to deserialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error> {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Serialization and deserialization for the MBTA date format.
pub mod mbta_date_format {
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Date string format.
    pub const FORMAT: &str = "%FT%T";

    /// Serialize an MBTA date.
    ///
    /// # Arguments
    ///
    /// * `date` - the date
    /// * `serializer` - the serializer
    pub fn serialize<S: Serializer>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", date.format(FORMAT)))
    }

    /// Attempt to deserialize an MBTA date.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
        let s = format!("{}T00:00:00", String::deserialize(deserializer)?);
        NaiveDateTime::parse_from_str(&s, FORMAT).map(|dt| dt.date()).map_err(serde::de::Error::custom)
    }
}

/// Serialization and deserialization for an optional MBTA datetime format.
pub mod optional_mbta_datetime_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::mbta_datetime_format::{serialize as date_serialize, FORMAT};

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
            Some(d) => date_serialize(d, serializer),
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
                let date = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

/// Serialization and deserialization for an optional MBTA date format.
pub mod optional_mbta_date_format {
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::mbta_date_format::{serialize as date_serialize, FORMAT};

    /// Serialize an optional MBTA date.
    ///
    /// # Arguments
    ///
    /// * `date` - the optional date
    /// * `serializer` - the serializer
    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let date = NaiveDateTime::parse_from_str(&format!("{}T00:00:00", s), FORMAT)
                    .map(|dt| dt.date())
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

/// Serialization and deserialization for an vector of MBTA dates format.
pub mod vec_mbta_date_format {
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Date string format.
    pub const FORMAT: &str = "%FT%T";

    /// Serialize a vector of MBTA dates.
    ///
    /// # Arguments
    ///
    /// * `dates` - the dates
    /// * `serializer` - the serializer
    pub fn serialize<S>(dates: &[NaiveDate], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(dates.iter().map(|dt| format!("{}", dt.format(FORMAT))))
    }

    /// Attempt to deserialize an optional MBTA dates.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::<String>::deserialize(deserializer)?;
        let mut dates = Vec::new();
        for dt in v {
            dates.push(
                NaiveDateTime::parse_from_str(&format!("{}T00:00:00", dt), FORMAT)
                    .map(|dt| dt.date())
                    .map_err(serde::de::Error::custom)?,
            )
        }
        Ok(dates)
    }
}
