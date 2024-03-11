// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing the Datadog log-based metric to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsMetricCreateAttributes {
    /// The compute rule to compute the log-based metric.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::LogsMetricCompute,
    /// The log-based metric filter. Logs matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::LogsMetricFilter>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::LogsMetricGroupBy>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsMetricCreateAttributes {
    pub fn new(compute: crate::datadogV2::model::LogsMetricCompute) -> LogsMetricCreateAttributes {
        LogsMetricCreateAttributes {
            compute,
            filter: None,
            group_by: None,
            _unparsed: false,
        }
    }

    pub fn filter(&mut self, value: crate::datadogV2::model::LogsMetricFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(
        &mut self,
        value: Vec<crate::datadogV2::model::LogsMetricGroupBy>,
    ) -> &mut Self {
        self.group_by = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsMetricCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsMetricCreateAttributesVisitor;
        impl<'a> Visitor<'a> for LogsMetricCreateAttributesVisitor {
            type Value = LogsMetricCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV2::model::LogsMetricCompute> = None;
                let mut filter: Option<crate::datadogV2::model::LogsMetricFilter> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::LogsMetricGroupBy>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
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
                        &_ => {}
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;

                let content = LogsMetricCreateAttributes {
                    compute,
                    filter,
                    group_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsMetricCreateAttributesVisitor)
    }
}
