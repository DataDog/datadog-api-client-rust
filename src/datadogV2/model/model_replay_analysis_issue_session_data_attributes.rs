// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a session related to a RUM replay analysis issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisIssueSessionDataAttributes {
    /// Session start timestamp in milliseconds.
    #[serde(rename = "session_start_timestamp_ms")]
    pub session_start_timestamp_ms: i64,
    /// List of signals observed in this session.
    #[serde(rename = "signals")]
    pub signals: Vec<crate::datadogV2::model::ReplayAnalysisSignal>,
    /// Name of the view where the issue was observed.
    #[serde(rename = "view_name")]
    pub view_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisIssueSessionDataAttributes {
    pub fn new(
        session_start_timestamp_ms: i64,
        signals: Vec<crate::datadogV2::model::ReplayAnalysisSignal>,
        view_name: String,
    ) -> ReplayAnalysisIssueSessionDataAttributes {
        ReplayAnalysisIssueSessionDataAttributes {
            session_start_timestamp_ms,
            signals,
            view_name,
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

impl<'de> Deserialize<'de> for ReplayAnalysisIssueSessionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisIssueSessionDataAttributesVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisIssueSessionDataAttributesVisitor {
            type Value = ReplayAnalysisIssueSessionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut session_start_timestamp_ms: Option<i64> = None;
                let mut signals: Option<Vec<crate::datadogV2::model::ReplayAnalysisSignal>> = None;
                let mut view_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "session_start_timestamp_ms" => {
                            session_start_timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signals" => {
                            signals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let session_start_timestamp_ms = session_start_timestamp_ms
                    .ok_or_else(|| M::Error::missing_field("session_start_timestamp_ms"))?;
                let signals = signals.ok_or_else(|| M::Error::missing_field("signals"))?;
                let view_name = view_name.ok_or_else(|| M::Error::missing_field("view_name"))?;

                let content = ReplayAnalysisIssueSessionDataAttributes {
                    session_start_timestamp_ms,
                    signals,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisIssueSessionDataAttributesVisitor)
    }
}
