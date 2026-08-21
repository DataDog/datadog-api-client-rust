// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One row of the retention grid, holding the results for a single cohort.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionGridCohort {
    /// The cells of the row, one per return period.
    #[serde(rename = "cells")]
    pub cells: Option<Vec<crate::datadogV2::model::ProductAnalyticsRetentionGridCohortCell>>,
    /// End of the cohort window, in epoch milliseconds.
    #[serde(rename = "cohort_end_time")]
    pub cohort_end_time: Option<i64>,
    /// Zero-based index of the cohort in the grid.
    #[serde(rename = "cohort_index")]
    pub cohort_index: Option<i64>,
    /// Number of entities in the cohort.
    #[serde(rename = "cohort_size")]
    pub cohort_size: Option<i64>,
    /// Start of the cohort window, in epoch milliseconds.
    #[serde(rename = "cohort_start_time")]
    pub cohort_start_time: Option<i64>,
    /// The group-by facet values that identify this row.
    #[serde(rename = "group_tags")]
    pub group_tags: Option<Vec<String>>,
    /// Label identifying the cohort, such as the week it started.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether the row holds one cohort's own numbers, or the weighted roll-up across every cohort.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType>,
    /// Unit definitions for the cell values.
    #[serde(rename = "unit")]
    pub unit: Option<Vec<crate::datadogV2::model::ProductAnalyticsUnit>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionGridCohort {
    pub fn new() -> ProductAnalyticsRetentionGridCohort {
        ProductAnalyticsRetentionGridCohort {
            cells: None,
            cohort_end_time: None,
            cohort_index: None,
            cohort_size: None,
            cohort_start_time: None,
            group_tags: None,
            name: None,
            type_: None,
            unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cells(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsRetentionGridCohortCell>,
    ) -> Self {
        self.cells = Some(value);
        self
    }

    pub fn cohort_end_time(mut self, value: i64) -> Self {
        self.cohort_end_time = Some(value);
        self
    }

    pub fn cohort_index(mut self, value: i64) -> Self {
        self.cohort_index = Some(value);
        self
    }

    pub fn cohort_size(mut self, value: i64) -> Self {
        self.cohort_size = Some(value);
        self
    }

    pub fn cohort_start_time(mut self, value: i64) -> Self {
        self.cohort_start_time = Some(value);
        self
    }

    pub fn group_tags(mut self, value: Vec<String>) -> Self {
        self.group_tags = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn unit(mut self, value: Vec<crate::datadogV2::model::ProductAnalyticsUnit>) -> Self {
        self.unit = Some(value);
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

impl Default for ProductAnalyticsRetentionGridCohort {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionGridCohort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionGridCohortVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionGridCohortVisitor {
            type Value = ProductAnalyticsRetentionGridCohort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cells: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsRetentionGridCohortCell>,
                > = None;
                let mut cohort_end_time: Option<i64> = None;
                let mut cohort_index: Option<i64> = None;
                let mut cohort_size: Option<i64> = None;
                let mut cohort_start_time: Option<i64> = None;
                let mut group_tags: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionGridCohortType,
                > = None;
                let mut unit: Option<Vec<crate::datadogV2::model::ProductAnalyticsUnit>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cells" => {
                            if v.is_null() {
                                continue;
                            }
                            cells = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cohort_end_time" => {
                            if v.is_null() {
                                continue;
                            }
                            cohort_end_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cohort_index" => {
                            if v.is_null() {
                                continue;
                            }
                            cohort_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cohort_size" => {
                            if v.is_null() {
                                continue;
                            }
                            cohort_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cohort_start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            cohort_start_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            group_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsRetentionGridCohort {
                    cells,
                    cohort_end_time,
                    cohort_index,
                    cohort_size,
                    cohort_start_time,
                    group_tags,
                    name,
                    type_,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionGridCohortVisitor)
    }
}
