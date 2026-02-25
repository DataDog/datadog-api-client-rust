// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsTimeseriesResponseAttributes {
    #[serde(rename = "intervals")]
    pub intervals: Option<Vec<crate::datadogV2::model::ProductAnalyticsInterval>>,
    #[serde(rename = "series")]
    pub series: Option<Vec<crate::datadogV2::model::ProductAnalyticsSerie>>,
    /// Timestamps for each data point (epoch milliseconds).
    #[serde(rename = "times")]
    pub times: Option<Vec<i64>>,
    /// Values for each series at each time point.
    #[serde(rename = "values")]
    pub values: Option<Vec<Vec<Option<f64>>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsTimeseriesResponseAttributes {
    pub fn new() -> ProductAnalyticsTimeseriesResponseAttributes {
        ProductAnalyticsTimeseriesResponseAttributes {
            intervals: None,
            series: None,
            times: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn intervals(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsInterval>,
    ) -> Self {
        self.intervals = Some(value);
        self
    }

    pub fn series(mut self, value: Vec<crate::datadogV2::model::ProductAnalyticsSerie>) -> Self {
        self.series = Some(value);
        self
    }

    pub fn times(mut self, value: Vec<i64>) -> Self {
        self.times = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Vec<Option<f64>>>) -> Self {
        self.values = Some(value);
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

impl Default for ProductAnalyticsTimeseriesResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsTimeseriesResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsTimeseriesResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsTimeseriesResponseAttributesVisitor {
            type Value = ProductAnalyticsTimeseriesResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut intervals: Option<Vec<crate::datadogV2::model::ProductAnalyticsInterval>> =
                    None;
                let mut series: Option<Vec<crate::datadogV2::model::ProductAnalyticsSerie>> = None;
                let mut times: Option<Vec<i64>> = None;
                let mut values: Option<Vec<Vec<Option<f64>>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "intervals" => {
                            if v.is_null() {
                                continue;
                            }
                            intervals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "series" => {
                            if v.is_null() {
                                continue;
                            }
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "times" => {
                            if v.is_null() {
                                continue;
                            }
                            times = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsTimeseriesResponseAttributes {
                    intervals,
                    series,
                    times,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsTimeseriesResponseAttributesVisitor)
    }
}
