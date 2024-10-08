// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The log-based metric properties that will be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsMetricUpdateAttributes {
    /// The compute rule to compute the log-based metric.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV2::model::LogsMetricUpdateCompute>,
    /// The log-based metric filter. Logs matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::LogsMetricFilter>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::LogsMetricGroupBy>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsMetricUpdateAttributes {
    pub fn new() -> LogsMetricUpdateAttributes {
        LogsMetricUpdateAttributes {
            compute: None,
            filter: None,
            group_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compute(mut self, value: crate::datadogV2::model::LogsMetricUpdateCompute) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::LogsMetricFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV2::model::LogsMetricGroupBy>) -> Self {
        self.group_by = Some(value);
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

impl Default for LogsMetricUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsMetricUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsMetricUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for LogsMetricUpdateAttributesVisitor {
            type Value = LogsMetricUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV2::model::LogsMetricUpdateCompute> = None;
                let mut filter: Option<crate::datadogV2::model::LogsMetricFilter> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::LogsMetricGroupBy>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LogsMetricUpdateAttributes {
                    compute,
                    filter,
                    group_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsMetricUpdateAttributesVisitor)
    }
}
