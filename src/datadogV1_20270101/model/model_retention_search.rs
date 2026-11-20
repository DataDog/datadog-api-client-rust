// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionSearch {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cohort_criteria")]
    pub cohort_criteria: crate::datadogV1_20270101::model::RetentionSearchCohortCriteria,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filters")]
    pub filters: Option<crate::datadogV1_20270101::model::RetentionSearchFilters>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "retention_entity")]
    pub retention_entity: crate::datadogV1_20270101::model::RetentionSearchRetentionEntity,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "return_condition")]
    pub return_condition: crate::datadogV1_20270101::model::RetentionSearchReturnCondition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "return_criteria")]
    pub return_criteria: Option<crate::datadogV1_20270101::model::RetentionSearchReturnCriteria>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RetentionSearch {
    pub fn new(cohort_criteria: crate::datadogV1_20270101::model::RetentionSearchCohortCriteria, retention_entity: crate::datadogV1_20270101::model::RetentionSearchRetentionEntity, return_condition: crate::datadogV1_20270101::model::RetentionSearchReturnCondition) -> RetentionSearch {
        RetentionSearch {
            cohort_criteria,
            filters: None,
            retention_entity,
            return_condition,
            return_criteria: None,
            _unparsed: false,
        }
    }
    
    pub fn filters(mut self, value: crate::datadogV1_20270101::model::RetentionSearchFilters) -> Self {
        self.filters = Some(value);
        self
    }
    
    pub fn return_criteria(mut self, value: crate::datadogV1_20270101::model::RetentionSearchReturnCriteria) -> Self {
        self.return_criteria = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for RetentionSearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionSearchVisitor;
        impl<'a> Visitor<'a> for RetentionSearchVisitor {
            type Value = RetentionSearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cohort_criteria: Option<crate::datadogV1_20270101::model::RetentionSearchCohortCriteria> = None;
                let mut filters: Option<crate::datadogV1_20270101::model::RetentionSearchFilters> = None;
                let mut retention_entity: Option<crate::datadogV1_20270101::model::RetentionSearchRetentionEntity> = None;
                let mut return_condition: Option<crate::datadogV1_20270101::model::RetentionSearchReturnCondition> = None;
                let mut return_criteria: Option<crate::datadogV1_20270101::model::RetentionSearchReturnCriteria> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cohort_criteria" => {
                            cohort_criteria = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "retention_entity" => {
                            retention_entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _retention_entity) = retention_entity {
                                match _retention_entity {
                                    crate::datadogV1_20270101::model::RetentionSearchRetentionEntity::UnparsedObject(_retention_entity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "return_condition" => {
                            return_condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _return_condition) = return_condition {
                                match _return_condition {
                                    crate::datadogV1_20270101::model::RetentionSearchReturnCondition::UnparsedObject(_return_condition) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "return_criteria" => {
                            if v.is_null() {
                                continue;
                            }
                            return_criteria = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let cohort_criteria = cohort_criteria.ok_or_else(|| M::Error::missing_field("cohort_criteria"))?;
                let retention_entity = retention_entity.ok_or_else(|| M::Error::missing_field("retention_entity"))?;
                let return_condition = return_condition.ok_or_else(|| M::Error::missing_field("return_condition"))?;

                let content = RetentionSearch {
                    cohort_criteria,
                    filters,
                    retention_entity,
                    return_condition,
                    return_criteria,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionSearchVisitor)
    }
}
