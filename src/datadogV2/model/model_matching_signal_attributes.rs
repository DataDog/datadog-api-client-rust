// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a matching security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchingSignalAttributes {
    /// The tracker ID linking the signal back to the originating event. Distinct from `id`, which identifies the matching signal itself.
    #[serde(rename = "event_tracker_id")]
    pub event_tracker_id: String,
    /// The severity of the signal.
    #[serde(rename = "severity")]
    pub severity: String,
    /// The title of the signal.
    #[serde(rename = "title")]
    pub title: String,
    /// The Unix timestamp (in milliseconds) at which the signal was triggered.
    #[serde(rename = "trigger_time_ms")]
    pub trigger_time_ms: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MatchingSignalAttributes {
    pub fn new(
        event_tracker_id: String,
        severity: String,
        title: String,
        trigger_time_ms: i64,
    ) -> MatchingSignalAttributes {
        MatchingSignalAttributes {
            event_tracker_id,
            severity,
            title,
            trigger_time_ms,
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

impl<'de> Deserialize<'de> for MatchingSignalAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MatchingSignalAttributesVisitor;
        impl<'a> Visitor<'a> for MatchingSignalAttributesVisitor {
            type Value = MatchingSignalAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_tracker_id: Option<String> = None;
                let mut severity: Option<String> = None;
                let mut title: Option<String> = None;
                let mut trigger_time_ms: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_tracker_id" => {
                            event_tracker_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_time_ms" => {
                            trigger_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let event_tracker_id =
                    event_tracker_id.ok_or_else(|| M::Error::missing_field("event_tracker_id"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let trigger_time_ms =
                    trigger_time_ms.ok_or_else(|| M::Error::missing_field("trigger_time_ms"))?;

                let content = MatchingSignalAttributes {
                    event_tracker_id,
                    severity,
                    title,
                    trigger_time_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MatchingSignalAttributesVisitor)
    }
}
