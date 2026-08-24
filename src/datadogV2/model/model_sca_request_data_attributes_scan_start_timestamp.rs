// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The time when the SCA scan started.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesScanStartTimestamp {
    /// Non-negative fractions of a second at nanosecond resolution.
    #[serde(rename = "nanos")]
    pub nanos: Option<i32>,
    /// Seconds of UTC time since Unix epoch.
    #[serde(rename = "seconds")]
    pub seconds: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesScanStartTimestamp {
    pub fn new() -> ScaRequestDataAttributesScanStartTimestamp {
        ScaRequestDataAttributesScanStartTimestamp {
            nanos: None,
            seconds: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn nanos(mut self, value: i32) -> Self {
        self.nanos = Some(value);
        self
    }

    pub fn seconds(mut self, value: i64) -> Self {
        self.seconds = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for ScaRequestDataAttributesScanStartTimestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesScanStartTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesScanStartTimestampVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesScanStartTimestampVisitor {
            type Value = ScaRequestDataAttributesScanStartTimestamp;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut nanos: Option<i32> = None;
                let mut seconds: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "nanos" => {
                            if v.is_null() {
                                continue;
                            }
                            nanos = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            seconds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesScanStartTimestamp {
                    nanos,
                    seconds,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesScanStartTimestampVisitor)
    }
}
