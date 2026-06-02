// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated high script evaluation detection grouped by source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedHighScriptEval {
    /// Average script evaluation duration in nanoseconds.
    #[serde(rename = "avg_duration")]
    pub avg_duration: i64,
    /// Average forced style/layout duration in nanoseconds.
    #[serde(rename = "avg_forced_style_layout")]
    pub avg_forced_style_layout: i64,
    /// Unique fingerprint identifying this detection group.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Impact score combining view frequency and duration severity.
    #[serde(rename = "impact_score")]
    pub impact_score: f64,
    /// Total number of detection instances across sampled views.
    #[serde(rename = "instance_count")]
    pub instance_count: i32,
    /// Type of invoker that triggered the script evaluation.
    #[serde(rename = "invoker_type")]
    pub invoker_type: String,
    /// Category of the script source.
    #[serialize_always]
    #[serde(rename = "source_category")]
    pub source_category: Option<String>,
    /// Name of the function that triggered the high script evaluation.
    #[serde(rename = "source_function_name")]
    pub source_function_name: String,
    /// URL of the script that triggered the high script evaluation.
    #[serialize_always]
    #[serde(rename = "source_url")]
    pub source_url: Option<String>,
    /// Number of sampled views where this detection occurred.
    #[serde(rename = "view_occurrences")]
    pub view_occurrences: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedHighScriptEval {
    pub fn new(
        avg_duration: i64,
        avg_forced_style_layout: i64,
        fingerprint: String,
        impact_score: f64,
        instance_count: i32,
        invoker_type: String,
        source_category: Option<String>,
        source_function_name: String,
        source_url: Option<String>,
        view_occurrences: i32,
    ) -> AggregatedHighScriptEval {
        AggregatedHighScriptEval {
            avg_duration,
            avg_forced_style_layout,
            fingerprint,
            impact_score,
            instance_count,
            invoker_type,
            source_category,
            source_function_name,
            source_url,
            view_occurrences,
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

impl<'de> Deserialize<'de> for AggregatedHighScriptEval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedHighScriptEvalVisitor;
        impl<'a> Visitor<'a> for AggregatedHighScriptEvalVisitor {
            type Value = AggregatedHighScriptEval;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_duration: Option<i64> = None;
                let mut avg_forced_style_layout: Option<i64> = None;
                let mut fingerprint: Option<String> = None;
                let mut impact_score: Option<f64> = None;
                let mut instance_count: Option<i32> = None;
                let mut invoker_type: Option<String> = None;
                let mut source_category: Option<Option<String>> = None;
                let mut source_function_name: Option<String> = None;
                let mut source_url: Option<Option<String>> = None;
                let mut view_occurrences: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_duration" => {
                            avg_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_forced_style_layout" => {
                            avg_forced_style_layout =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fingerprint" => {
                            fingerprint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_score" => {
                            impact_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_count" => {
                            instance_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invoker_type" => {
                            invoker_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_category" => {
                            source_category =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_function_name" => {
                            source_function_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_url" => {
                            source_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_occurrences" => {
                            view_occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let avg_duration =
                    avg_duration.ok_or_else(|| M::Error::missing_field("avg_duration"))?;
                let avg_forced_style_layout = avg_forced_style_layout
                    .ok_or_else(|| M::Error::missing_field("avg_forced_style_layout"))?;
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let impact_score =
                    impact_score.ok_or_else(|| M::Error::missing_field("impact_score"))?;
                let instance_count =
                    instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let invoker_type =
                    invoker_type.ok_or_else(|| M::Error::missing_field("invoker_type"))?;
                let source_category =
                    source_category.ok_or_else(|| M::Error::missing_field("source_category"))?;
                let source_function_name = source_function_name
                    .ok_or_else(|| M::Error::missing_field("source_function_name"))?;
                let source_url = source_url.ok_or_else(|| M::Error::missing_field("source_url"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = AggregatedHighScriptEval {
                    avg_duration,
                    avg_forced_style_layout,
                    fingerprint,
                    impact_score,
                    instance_count,
                    invoker_type,
                    source_category,
                    source_function_name,
                    source_url,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedHighScriptEvalVisitor)
    }
}
