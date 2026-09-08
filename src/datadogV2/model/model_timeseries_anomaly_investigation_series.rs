// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Logical series on which the anomaly was detected.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationSeries {
    /// Tags identifying the selected group. Empty for a query without grouping.
    #[serde(rename = "group_tags")]
    pub group_tags: Vec<String>,
    /// Display label for the selected series.
    #[serde(rename = "label")]
    pub label: String,
    /// Zero-based index of the caller's formula that produced the series.
    #[serde(rename = "query_index")]
    pub query_index: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationSeries {
    pub fn new(
        group_tags: Vec<String>,
        label: String,
        query_index: i64,
    ) -> TimeseriesAnomalyInvestigationSeries {
        TimeseriesAnomalyInvestigationSeries {
            group_tags,
            label,
            query_index,
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationSeriesVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationSeriesVisitor {
            type Value = TimeseriesAnomalyInvestigationSeries;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_tags: Option<Vec<String>> = None;
                let mut label: Option<String> = None;
                let mut query_index: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_tags" => {
                            group_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_index" => {
                            query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let group_tags = group_tags.ok_or_else(|| M::Error::missing_field("group_tags"))?;
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;
                let query_index =
                    query_index.ok_or_else(|| M::Error::missing_field("query_index"))?;

                let content = TimeseriesAnomalyInvestigationSeries {
                    group_tags,
                    label,
                    query_index,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationSeriesVisitor)
    }
}
