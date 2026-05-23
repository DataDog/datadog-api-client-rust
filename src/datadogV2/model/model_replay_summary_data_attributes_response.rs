// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a RUM replay summary response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplaySummaryDataAttributesResponse {
    /// List of chapters breaking down the session into distinct activity segments.
    #[serde(rename = "chapters")]
    pub chapters: Vec<crate::datadogV2::model::ReplaySummaryChapter>,
    /// Whether the session contained sufficient user activity to generate a summary.
    #[serde(rename = "has_enough_activity")]
    pub has_enough_activity: bool,
    /// Whether the session exceeded the event count limit for summary generation.
    #[serde(rename = "has_too_many_events")]
    pub has_too_many_events: bool,
    /// AI-generated summary of the replay session.
    #[serde(rename = "summary")]
    pub summary: String,
    /// Version of the prompt used to generate the summary.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplaySummaryDataAttributesResponse {
    pub fn new(
        chapters: Vec<crate::datadogV2::model::ReplaySummaryChapter>,
        has_enough_activity: bool,
        has_too_many_events: bool,
        summary: String,
        version: String,
    ) -> ReplaySummaryDataAttributesResponse {
        ReplaySummaryDataAttributesResponse {
            chapters,
            has_enough_activity,
            has_too_many_events,
            summary,
            version,
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

impl<'de> Deserialize<'de> for ReplaySummaryDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplaySummaryDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for ReplaySummaryDataAttributesResponseVisitor {
            type Value = ReplaySummaryDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chapters: Option<Vec<crate::datadogV2::model::ReplaySummaryChapter>> = None;
                let mut has_enough_activity: Option<bool> = None;
                let mut has_too_many_events: Option<bool> = None;
                let mut summary: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chapters" => {
                            chapters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_enough_activity" => {
                            has_enough_activity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_too_many_events" => {
                            has_too_many_events =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let chapters = chapters.ok_or_else(|| M::Error::missing_field("chapters"))?;
                let has_enough_activity = has_enough_activity
                    .ok_or_else(|| M::Error::missing_field("has_enough_activity"))?;
                let has_too_many_events = has_too_many_events
                    .ok_or_else(|| M::Error::missing_field("has_too_many_events"))?;
                let summary = summary.ok_or_else(|| M::Error::missing_field("summary"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = ReplaySummaryDataAttributesResponse {
                    chapters,
                    has_enough_activity,
                    has_too_many_events,
                    summary,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplaySummaryDataAttributesResponseVisitor)
    }
}
