// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Latest inspection of the repository for the tracking plan instrumentation. A new analysis replaces the stored
/// one.
/// Events and properties that it does not list are not verified.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchCodeAnalysis {
    /// Time of the analysis. Set by the server.
    #[serde(rename = "analyzed_at")]
    pub analyzed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Results for the events of the tracking plan. List each event once.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchEventFinding>>,
    /// Code revision that was inspected, such as a commit SHA.
    #[serde(rename = "revision")]
    pub revision: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchCodeAnalysis {
    pub fn new() -> ProductAnalyticsLaunchCodeAnalysis {
        ProductAnalyticsLaunchCodeAnalysis {
            analyzed_at: None,
            events: None,
            revision: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn analyzed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.analyzed_at = Some(value);
        self
    }

    pub fn events(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsLaunchEventFinding>,
    ) -> Self {
        self.events = Some(value);
        self
    }

    pub fn revision(mut self, value: String) -> Self {
        self.revision = Some(value);
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

impl Default for ProductAnalyticsLaunchCodeAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchCodeAnalysis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchCodeAnalysisVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchCodeAnalysisVisitor {
            type Value = ProductAnalyticsLaunchCodeAnalysis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analyzed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut events: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsLaunchEventFinding>,
                > = None;
                let mut revision: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "analyzed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            analyzed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revision" => {
                            if v.is_null() {
                                continue;
                            }
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsLaunchCodeAnalysis {
                    analyzed_at,
                    events,
                    revision,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchCodeAnalysisVisitor)
    }
}
