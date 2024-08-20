// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The hourly usage of timeseries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageTimeseriesHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// Contains the number of custom metrics that are inputs for aggregations (metric configured is custom).
    #[serde(rename = "num_custom_input_timeseries")]
    pub num_custom_input_timeseries: Option<i64>,
    /// Contains the number of custom metrics that are outputs for aggregations (metric configured is custom).
    #[serde(rename = "num_custom_output_timeseries")]
    pub num_custom_output_timeseries: Option<i64>,
    /// Contains sum of non-aggregation custom metrics and custom metrics that are outputs for aggregations.
    #[serde(rename = "num_custom_timeseries")]
    pub num_custom_timeseries: Option<i64>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageTimeseriesHour {
    pub fn new() -> UsageTimeseriesHour {
        UsageTimeseriesHour {
            hour: None,
            num_custom_input_timeseries: None,
            num_custom_output_timeseries: None,
            num_custom_timeseries: None,
            org_name: None,
            public_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn num_custom_input_timeseries(mut self, value: i64) -> Self {
        self.num_custom_input_timeseries = Some(value);
        self
    }

    pub fn num_custom_output_timeseries(mut self, value: i64) -> Self {
        self.num_custom_output_timeseries = Some(value);
        self
    }

    pub fn num_custom_timeseries(mut self, value: i64) -> Self {
        self.num_custom_timeseries = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
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

impl Default for UsageTimeseriesHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageTimeseriesHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageTimeseriesHourVisitor;
        impl<'a> Visitor<'a> for UsageTimeseriesHourVisitor {
            type Value = UsageTimeseriesHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut num_custom_input_timeseries: Option<i64> = None;
                let mut num_custom_output_timeseries: Option<i64> = None;
                let mut num_custom_timeseries: Option<i64> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_custom_input_timeseries" => {
                            if v.is_null() {
                                continue;
                            }
                            num_custom_input_timeseries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_custom_output_timeseries" => {
                            if v.is_null() {
                                continue;
                            }
                            num_custom_output_timeseries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_custom_timeseries" => {
                            if v.is_null() {
                                continue;
                            }
                            num_custom_timeseries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageTimeseriesHour {
                    hour,
                    num_custom_input_timeseries,
                    num_custom_output_timeseries,
                    num_custom_timeseries,
                    org_name,
                    public_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageTimeseriesHourVisitor)
    }
}
