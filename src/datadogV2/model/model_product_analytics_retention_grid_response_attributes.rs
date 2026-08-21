// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a retention grid response, containing the cohort rows and the period columns.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionGridResponseAttributes {
    /// The cohorts forming the rows of the grid.
    #[serde(rename = "cohorts")]
    pub cohorts: Option<Vec<crate::datadogV2::model::ProductAnalyticsRetentionGridCohort>>,
    /// The entity whose retention was measured.
    #[serde(rename = "retention_entity")]
    pub retention_entity: Option<String>,
    /// The return periods forming the columns of the grid.
    #[serde(rename = "retention_periods")]
    pub retention_periods: Option<Vec<crate::datadogV2::model::ProductAnalyticsRetentionPeriod>>,
    /// Unit definitions for the grid values.
    #[serde(rename = "unit")]
    pub unit: Option<Vec<crate::datadogV2::model::ProductAnalyticsUnit>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionGridResponseAttributes {
    pub fn new() -> ProductAnalyticsRetentionGridResponseAttributes {
        ProductAnalyticsRetentionGridResponseAttributes {
            cohorts: None,
            retention_entity: None,
            retention_periods: None,
            unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cohorts(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsRetentionGridCohort>,
    ) -> Self {
        self.cohorts = Some(value);
        self
    }

    pub fn retention_entity(mut self, value: String) -> Self {
        self.retention_entity = Some(value);
        self
    }

    pub fn retention_periods(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsRetentionPeriod>,
    ) -> Self {
        self.retention_periods = Some(value);
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

impl Default for ProductAnalyticsRetentionGridResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionGridResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionGridResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionGridResponseAttributesVisitor {
            type Value = ProductAnalyticsRetentionGridResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cohorts: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsRetentionGridCohort>,
                > = None;
                let mut retention_entity: Option<String> = None;
                let mut retention_periods: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsRetentionPeriod>,
                > = None;
                let mut unit: Option<Vec<crate::datadogV2::model::ProductAnalyticsUnit>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cohorts" => {
                            if v.is_null() {
                                continue;
                            }
                            cohorts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_entity" => {
                            if v.is_null() {
                                continue;
                            }
                            retention_entity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_periods" => {
                            if v.is_null() {
                                continue;
                            }
                            retention_periods =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ProductAnalyticsRetentionGridResponseAttributes {
                    cohorts,
                    retention_entity,
                    retention_periods,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionGridResponseAttributesVisitor)
    }
}
