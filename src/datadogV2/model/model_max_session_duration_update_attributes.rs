// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating maximum session duration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MaxSessionDurationUpdateAttributes {
    /// Maximum session duration in seconds. Must not exceed 30 days (2592000 seconds).
    /// Note: Government cloud environments are limited to 24 hours (86400 seconds).
    #[serde(rename = "max_session_duration")]
    pub max_session_duration: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MaxSessionDurationUpdateAttributes {
    pub fn new(max_session_duration: i64) -> MaxSessionDurationUpdateAttributes {
        MaxSessionDurationUpdateAttributes {
            max_session_duration,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for MaxSessionDurationUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaxSessionDurationUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for MaxSessionDurationUpdateAttributesVisitor {
            type Value = MaxSessionDurationUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_session_duration: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_session_duration" => {
                            max_session_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let max_session_duration = max_session_duration
                    .ok_or_else(|| M::Error::missing_field("max_session_duration"))?;

                let content = MaxSessionDurationUpdateAttributes {
                    max_session_duration,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MaxSessionDurationUpdateAttributesVisitor)
    }
}
