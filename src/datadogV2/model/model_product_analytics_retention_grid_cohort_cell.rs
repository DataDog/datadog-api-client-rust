// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One cell of the retention grid, holding the result for a single cohort over a single return period.
/// Aggregated rows omit the time and count fields.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionGridCohortCell {
    /// Number of entities that returned during the period.
    #[serde(rename = "cell_count")]
    pub cell_count: Option<i64>,
    /// Fraction of the cohort that returned, between `0` and `1`.
    #[serde(rename = "cell_rate")]
    pub cell_rate: Option<f64>,
    /// Change in the metric relative to the cohort baseline.
    #[serde(
        rename = "cell_relative_value_change",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cell_relative_value_change: Option<Option<f64>>,
    /// Value of the computed metric, when a metric other than the retention rate is requested.
    #[serde(
        rename = "cell_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cell_value: Option<Option<f64>>,
    /// Whether the return period is still open, so the numbers are not yet final.
    #[serde(rename = "is_partial_data")]
    pub is_partial_data: Option<bool>,
    /// End of the return period, in epoch milliseconds.
    #[serde(rename = "return_period_end_time")]
    pub return_period_end_time: Option<i64>,
    /// Zero-based index of the return period this cell belongs to.
    #[serde(rename = "return_period_index")]
    pub return_period_index: Option<i64>,
    /// Start of the return period, in epoch milliseconds.
    #[serde(rename = "return_period_start_time")]
    pub return_period_start_time: Option<i64>,
    /// Whether the row holds one cohort's own numbers, or the weighted roll-up across every cohort.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionGridCohortCell {
    pub fn new() -> ProductAnalyticsRetentionGridCohortCell {
        ProductAnalyticsRetentionGridCohortCell {
            cell_count: None,
            cell_rate: None,
            cell_relative_value_change: None,
            cell_value: None,
            is_partial_data: None,
            return_period_end_time: None,
            return_period_index: None,
            return_period_start_time: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cell_count(mut self, value: i64) -> Self {
        self.cell_count = Some(value);
        self
    }

    pub fn cell_rate(mut self, value: f64) -> Self {
        self.cell_rate = Some(value);
        self
    }

    pub fn cell_relative_value_change(mut self, value: Option<f64>) -> Self {
        self.cell_relative_value_change = Some(value);
        self
    }

    pub fn cell_value(mut self, value: Option<f64>) -> Self {
        self.cell_value = Some(value);
        self
    }

    pub fn is_partial_data(mut self, value: bool) -> Self {
        self.is_partial_data = Some(value);
        self
    }

    pub fn return_period_end_time(mut self, value: i64) -> Self {
        self.return_period_end_time = Some(value);
        self
    }

    pub fn return_period_index(mut self, value: i64) -> Self {
        self.return_period_index = Some(value);
        self
    }

    pub fn return_period_start_time(mut self, value: i64) -> Self {
        self.return_period_start_time = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType,
    ) -> Self {
        self.type_ = Some(value);
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

impl Default for ProductAnalyticsRetentionGridCohortCell {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionGridCohortCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionGridCohortCellVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionGridCohortCellVisitor {
            type Value = ProductAnalyticsRetentionGridCohortCell;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cell_count: Option<i64> = None;
                let mut cell_rate: Option<f64> = None;
                let mut cell_relative_value_change: Option<Option<f64>> = None;
                let mut cell_value: Option<Option<f64>> = None;
                let mut is_partial_data: Option<bool> = None;
                let mut return_period_end_time: Option<i64> = None;
                let mut return_period_index: Option<i64> = None;
                let mut return_period_start_time: Option<i64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cell_count" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cell_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cell_rate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cell_relative_value_change" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            cell_relative_value_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cell_value" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            cell_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_partial_data" => {
                            if v.is_null() {
                                continue;
                            }
                            is_partial_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "return_period_end_time" => {
                            if v.is_null() {
                                continue;
                            }
                            return_period_end_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "return_period_index" => {
                            if v.is_null() {
                                continue;
                            }
                            return_period_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "return_period_start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            return_period_start_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsRetentionGridCohortCell {
                    cell_count,
                    cell_rate,
                    cell_relative_value_change,
                    cell_value,
                    is_partial_data,
                    return_period_end_time,
                    return_period_index,
                    return_period_start_time,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionGridCohortCellVisitor)
    }
}
