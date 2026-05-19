// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary statistics for a metric recorded during a Model Lab run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabMetricSummary {
    /// The total number of recorded values.
    #[serde(rename = "count")]
    pub count: i64,
    /// The first step at which the metric was recorded.
    #[serde(
        rename = "first_step",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub first_step: Option<Option<i64>>,
    /// The metric name.
    #[serde(rename = "key")]
    pub key: String,
    /// The last step at which the metric was recorded.
    #[serde(
        rename = "last_step",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_step: Option<Option<i64>>,
    /// The most recently recorded value.
    #[serde(rename = "latest", default, with = "::serde_with::rust::double_option")]
    pub latest: Option<Option<f64>>,
    /// The maximum recorded value.
    #[serde(rename = "max", default, with = "::serde_with::rust::double_option")]
    pub max: Option<Option<f64>>,
    /// The mean of recorded values.
    #[serde(rename = "mean", default, with = "::serde_with::rust::double_option")]
    pub mean: Option<Option<f64>>,
    /// The minimum recorded value.
    #[serde(rename = "min", default, with = "::serde_with::rust::double_option")]
    pub min: Option<Option<f64>>,
    /// The standard deviation of recorded values.
    #[serde(rename = "stddev", default, with = "::serde_with::rust::double_option")]
    pub stddev: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabMetricSummary {
    pub fn new(count: i64, key: String) -> ModelLabMetricSummary {
        ModelLabMetricSummary {
            count,
            first_step: None,
            key,
            last_step: None,
            latest: None,
            max: None,
            mean: None,
            min: None,
            stddev: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn first_step(mut self, value: Option<i64>) -> Self {
        self.first_step = Some(value);
        self
    }

    pub fn last_step(mut self, value: Option<i64>) -> Self {
        self.last_step = Some(value);
        self
    }

    pub fn latest(mut self, value: Option<f64>) -> Self {
        self.latest = Some(value);
        self
    }

    pub fn max(mut self, value: Option<f64>) -> Self {
        self.max = Some(value);
        self
    }

    pub fn mean(mut self, value: Option<f64>) -> Self {
        self.mean = Some(value);
        self
    }

    pub fn min(mut self, value: Option<f64>) -> Self {
        self.min = Some(value);
        self
    }

    pub fn stddev(mut self, value: Option<f64>) -> Self {
        self.stddev = Some(value);
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

impl<'de> Deserialize<'de> for ModelLabMetricSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabMetricSummaryVisitor;
        impl<'a> Visitor<'a> for ModelLabMetricSummaryVisitor {
            type Value = ModelLabMetricSummary;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut first_step: Option<Option<i64>> = None;
                let mut key: Option<String> = None;
                let mut last_step: Option<Option<i64>> = None;
                let mut latest: Option<Option<f64>> = None;
                let mut max: Option<Option<f64>> = None;
                let mut mean: Option<Option<f64>> = None;
                let mut min: Option<Option<f64>> = None;
                let mut stddev: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_step" => {
                            first_step = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_step" => {
                            last_step = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "latest" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            latest = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mean" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            mean = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stddev" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            stddev = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;

                let content = ModelLabMetricSummary {
                    count,
                    first_step,
                    key,
                    last_step,
                    latest,
                    max,
                    mean,
                    min,
                    stddev,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabMetricSummaryVisitor)
    }
}
