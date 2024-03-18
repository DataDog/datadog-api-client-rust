// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Usage amount for a given usage type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HourlyUsageMeasurement {
    /// Type of usage.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<String>,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HourlyUsageMeasurement {
    pub fn new() -> HourlyUsageMeasurement {
        HourlyUsageMeasurement {
            usage_type: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn usage_type(mut self, value: String) -> Self {
        self.usage_type = Some(value);
        self
    }

    pub fn value(mut self, value: Option<i64>) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for HourlyUsageMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HourlyUsageMeasurement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HourlyUsageMeasurementVisitor;
        impl<'a> Visitor<'a> for HourlyUsageMeasurementVisitor {
            type Value = HourlyUsageMeasurement;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut usage_type: Option<String> = None;
                let mut value: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "usage_type" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HourlyUsageMeasurement {
                    usage_type,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HourlyUsageMeasurementVisitor)
    }
}
