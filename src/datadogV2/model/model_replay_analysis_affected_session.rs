// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A session affected by a replay analysis issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisAffectedSession {
    /// Unique identifier of the affected session.
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Session start timestamp in milliseconds.
    #[serde(
        rename = "session_start_timestamp_ms",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_start_timestamp_ms: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisAffectedSession {
    pub fn new(session_id: String) -> ReplayAnalysisAffectedSession {
        ReplayAnalysisAffectedSession {
            session_id,
            session_start_timestamp_ms: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn session_start_timestamp_ms(mut self, value: Option<i64>) -> Self {
        self.session_start_timestamp_ms = Some(value);
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

impl<'de> Deserialize<'de> for ReplayAnalysisAffectedSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisAffectedSessionVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisAffectedSessionVisitor {
            type Value = ReplayAnalysisAffectedSession;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut session_id: Option<String> = None;
                let mut session_start_timestamp_ms: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "session_id" => {
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_start_timestamp_ms" => {
                            session_start_timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let session_id = session_id.ok_or_else(|| M::Error::missing_field("session_id"))?;

                let content = ReplayAnalysisAffectedSession {
                    session_id,
                    session_start_timestamp_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisAffectedSessionVisitor)
    }
}
