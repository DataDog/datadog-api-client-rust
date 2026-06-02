// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A top long task invoker within an invoker type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopLongTaskInvoker {
    /// Number of sampled views where this invoker had long tasks contributing to the criteria metric.
    #[serde(rename = "criteria_view_occurrences")]
    pub criteria_view_occurrences: Option<i32>,
    /// Cleaned source file path for the invoker script.
    #[serialize_always]
    #[serde(rename = "file")]
    pub file: Option<String>,
    /// Rank-product impact score combining view frequency and blocking time severity.
    #[serde(rename = "impact_score")]
    pub impact_score: Option<f64>,
    /// Name of the invoker function or script.
    #[serde(rename = "invoker")]
    pub invoker: String,
    /// Statistical distributions of long task metrics computed per view across sampled views.
    #[serde(rename = "stats_per_view")]
    pub stats_per_view: crate::datadogV2::model::LongTaskStatsPerView,
    /// Number of sampled views where this invoker had any long tasks.
    #[serde(rename = "view_occurrences")]
    pub view_occurrences: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopLongTaskInvoker {
    pub fn new(
        file: Option<String>,
        invoker: String,
        stats_per_view: crate::datadogV2::model::LongTaskStatsPerView,
        view_occurrences: i32,
    ) -> TopLongTaskInvoker {
        TopLongTaskInvoker {
            criteria_view_occurrences: None,
            file,
            impact_score: None,
            invoker,
            stats_per_view,
            view_occurrences,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn criteria_view_occurrences(mut self, value: i32) -> Self {
        self.criteria_view_occurrences = Some(value);
        self
    }

    pub fn impact_score(mut self, value: f64) -> Self {
        self.impact_score = Some(value);
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

impl<'de> Deserialize<'de> for TopLongTaskInvoker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopLongTaskInvokerVisitor;
        impl<'a> Visitor<'a> for TopLongTaskInvokerVisitor {
            type Value = TopLongTaskInvoker;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut criteria_view_occurrences: Option<i32> = None;
                let mut file: Option<Option<String>> = None;
                let mut impact_score: Option<f64> = None;
                let mut invoker: Option<String> = None;
                let mut stats_per_view: Option<crate::datadogV2::model::LongTaskStatsPerView> =
                    None;
                let mut view_occurrences: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "criteria_view_occurrences" => {
                            if v.is_null() {
                                continue;
                            }
                            criteria_view_occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file" => {
                            file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_score" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            impact_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invoker" => {
                            invoker = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stats_per_view" => {
                            stats_per_view =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let file = file.ok_or_else(|| M::Error::missing_field("file"))?;
                let invoker = invoker.ok_or_else(|| M::Error::missing_field("invoker"))?;
                let stats_per_view =
                    stats_per_view.ok_or_else(|| M::Error::missing_field("stats_per_view"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = TopLongTaskInvoker {
                    criteria_view_occurrences,
                    file,
                    impact_score,
                    invoker,
                    stats_per_view,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopLongTaskInvokerVisitor)
    }
}
