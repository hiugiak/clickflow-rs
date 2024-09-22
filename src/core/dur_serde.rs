use std::{time::Duration, u64};

use serde::de::Visitor;

pub(super) fn serialize<S>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(dur.as_millis().try_into().unwrap())
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a duration in milliseconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.try_into() {
            Ok(u64_value) => Ok(Duration::from_millis(u64_value)),
            Err(e) => Err(E::custom(format!("{}: {}", e, v))),
        }
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.try_into() {
            Ok(u64_value) => Ok(Duration::from_millis(u64_value)),
            Err(e) => Err(E::custom(format!("{}: {}", e, v))),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Duration::from_millis(v))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.try_into() {
            Ok(u64_value) => Ok(Duration::from_millis(u64_value)),
            Err(e) => Err(E::custom(format!("{}: {}", e, v))),
        }
    }
}

pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_u64(DurationVisitor)
}
