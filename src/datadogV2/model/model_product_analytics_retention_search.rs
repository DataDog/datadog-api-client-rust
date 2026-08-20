// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the cohort and return criteria that make up a retention query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionSearch {
    /// Defines the event that places an entity into a cohort, and how cohorts are bucketed over time.
    #[serde(rename = "cohort_criteria")]
    pub cohort_criteria: crate::datadogV2::model::ProductAnalyticsRetentionCohortCriteria,
    /// Filters narrowing the events considered by a retention query.
    #[serde(rename = "filters")]
    pub filters: Option<crate::datadogV2::model::ProductAnalyticsRetentionFilters>,
    /// The entity whose retention is measured.
    #[serde(rename = "retention_entity")]
    pub retention_entity: crate::datadogV2::model::ProductAnalyticsRetentionEntity,
    /// When an entity counts as having returned. Use `conversion_on` to count only entities that
    /// returned during the period itself, or `conversion_on_or_after` to also count later returns.
    #[serde(rename = "return_condition")]
    pub return_condition: crate::datadogV2::model::ProductAnalyticsRetentionReturnCondition,
    /// Defines the event that counts as a return, and the window in which it must occur.
    #[serde(rename = "return_criteria")]
    pub return_criteria: Option<crate::datadogV2::model::ProductAnalyticsRetentionReturnCriteria>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionSearch {
    pub fn new(
        cohort_criteria: crate::datadogV2::model::ProductAnalyticsRetentionCohortCriteria,
        retention_entity: crate::datadogV2::model::ProductAnalyticsRetentionEntity,
        return_condition: crate::datadogV2::model::ProductAnalyticsRetentionReturnCondition,
    ) -> ProductAnalyticsRetentionSearch {
        ProductAnalyticsRetentionSearch {
            cohort_criteria,
            filters: None,
            retention_entity,
            return_condition,
            return_criteria: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filters(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsRetentionFilters,
    ) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn return_criteria(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsRetentionReturnCriteria,
    ) -> Self {
        self.return_criteria = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionSearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionSearchVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionSearchVisitor {
            type Value = ProductAnalyticsRetentionSearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cohort_criteria: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionCohortCriteria,
                > = None;
                let mut filters: Option<crate::datadogV2::model::ProductAnalyticsRetentionFilters> =
                    None;
                let mut retention_entity: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionEntity,
                > = None;
                let mut return_condition: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionReturnCondition,
                > = None;
                let mut return_criteria: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionReturnCriteria,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cohort_criteria" => {
                            cohort_criteria =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_entity" => {
                            retention_entity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _retention_entity) = retention_entity {
                                match _retention_entity {
                                    crate::datadogV2::model::ProductAnalyticsRetentionEntity::UnparsedObject(_retention_entity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "return_condition" => {
                            return_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _return_condition) = return_condition {
                                match _return_condition {
                                    crate::datadogV2::model::ProductAnalyticsRetentionReturnCondition::UnparsedObject(_return_condition) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "return_criteria" => {
                            if v.is_null() {
                                continue;
                            }
                            return_criteria =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cohort_criteria =
                    cohort_criteria.ok_or_else(|| M::Error::missing_field("cohort_criteria"))?;
                let retention_entity =
                    retention_entity.ok_or_else(|| M::Error::missing_field("retention_entity"))?;
                let return_condition =
                    return_condition.ok_or_else(|| M::Error::missing_field("return_condition"))?;

                let content = ProductAnalyticsRetentionSearch {
                    cohort_criteria,
                    filters,
                    retention_entity,
                    return_condition,
                    return_criteria,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionSearchVisitor)
    }
}
