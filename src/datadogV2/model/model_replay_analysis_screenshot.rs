// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A screenshot captured during a replay session.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisScreenshot {
    /// Filename or key identifier of the screenshot.
    #[serde(rename = "screenshot_key")]
    pub screenshot_key: String,
    /// Unique identifier of the session where the screenshot was taken.
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Timestamp of the screenshot in milliseconds.
    #[serde(rename = "timestamp_ms")]
    pub timestamp_ms: i64,
    /// Unique identifier of the view where the screenshot was taken.
    #[serde(rename = "view_id")]
    pub view_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisScreenshot {
    pub fn new(
        screenshot_key: String,
        session_id: String,
        timestamp_ms: i64,
        view_id: String,
    ) -> ReplayAnalysisScreenshot {
        ReplayAnalysisScreenshot {
            screenshot_key,
            session_id,
            timestamp_ms,
            view_id,
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

impl<'de> Deserialize<'de> for ReplayAnalysisScreenshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisScreenshotVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisScreenshotVisitor {
            type Value = ReplayAnalysisScreenshot;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut screenshot_key: Option<String> = None;
                let mut session_id: Option<String> = None;
                let mut timestamp_ms: Option<i64> = None;
                let mut view_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "screenshot_key" => {
                            screenshot_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_ms" => {
                            timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_id" => {
                            view_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let screenshot_key =
                    screenshot_key.ok_or_else(|| M::Error::missing_field("screenshot_key"))?;
                let session_id = session_id.ok_or_else(|| M::Error::missing_field("session_id"))?;
                let timestamp_ms =
                    timestamp_ms.ok_or_else(|| M::Error::missing_field("timestamp_ms"))?;
                let view_id = view_id.ok_or_else(|| M::Error::missing_field("view_id"))?;

                let content = ReplayAnalysisScreenshot {
                    screenshot_key,
                    session_id,
                    timestamp_ms,
                    view_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisScreenshotVisitor)
    }
}
