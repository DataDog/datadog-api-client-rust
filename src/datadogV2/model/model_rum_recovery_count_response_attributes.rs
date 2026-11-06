// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Approximate recoverable count.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveryCountResponseAttributes {
    /// Estimated distinct recoverable sessions not already retained in hot storage. The full estimate is returned without capping it at max_recoverable_sessions.
    #[serde(rename = "estimated_count")]
    pub estimated_count: i64,
    /// Configured maximum estimated recoverable sessions per batch (100,000 by default). Estimates equal to this value are accepted at creation.
    #[serde(rename = "max_recoverable_sessions")]
    pub max_recoverable_sessions: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveryCountResponseAttributes {
    pub fn new(
        estimated_count: i64,
        max_recoverable_sessions: i64,
    ) -> RumRecoveryCountResponseAttributes {
        RumRecoveryCountResponseAttributes {
            estimated_count,
            max_recoverable_sessions,
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

impl<'de> Deserialize<'de> for RumRecoveryCountResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveryCountResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoveryCountResponseAttributesVisitor {
            type Value = RumRecoveryCountResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut estimated_count: Option<i64> = None;
                let mut max_recoverable_sessions: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "estimated_count" => {
                            estimated_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_recoverable_sessions" => {
                            max_recoverable_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let estimated_count =
                    estimated_count.ok_or_else(|| M::Error::missing_field("estimated_count"))?;
                let max_recoverable_sessions = max_recoverable_sessions
                    .ok_or_else(|| M::Error::missing_field("max_recoverable_sessions"))?;

                let content = RumRecoveryCountResponseAttributes {
                    estimated_count,
                    max_recoverable_sessions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveryCountResponseAttributesVisitor)
    }
}
