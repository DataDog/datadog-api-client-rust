// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Narrows a retention query to a single cell, at the intersection of one cohort and one return period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionCellScope {
    /// Selects a cohort, either by index or by the aggregation that rolls all cohorts together.
    #[serde(rename = "cohort_target")]
    pub cohort_target: crate::datadogV2::model::ProductAnalyticsRetentionCohortTarget,
    /// Selects a cohort or return period by its zero-based position in the grid.
    #[serde(rename = "return_period_target")]
    pub return_period_target: crate::datadogV2::model::ProductAnalyticsRetentionIndexTarget,
    /// The discriminator identifying a scope narrowed to one grid cell.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ProductAnalyticsRetentionCellScopeType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionCellScope {
    pub fn new(
        cohort_target: crate::datadogV2::model::ProductAnalyticsRetentionCohortTarget,
        return_period_target: crate::datadogV2::model::ProductAnalyticsRetentionIndexTarget,
        type_: crate::datadogV2::model::ProductAnalyticsRetentionCellScopeType,
    ) -> ProductAnalyticsRetentionCellScope {
        ProductAnalyticsRetentionCellScope {
            cohort_target,
            return_period_target,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionCellScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionCellScopeVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionCellScopeVisitor {
            type Value = ProductAnalyticsRetentionCellScope;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cohort_target: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionCohortTarget,
                > = None;
                let mut return_period_target: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionIndexTarget,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionCellScopeType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cohort_target" => {
                            cohort_target =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cohort_target) = cohort_target {
                                match _cohort_target {
                                    crate::datadogV2::model::ProductAnalyticsRetentionCohortTarget::UnparsedObject(_cohort_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "return_period_target" => {
                            return_period_target =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ProductAnalyticsRetentionCellScopeType::UnparsedObject(_type_) => {
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
                let cohort_target =
                    cohort_target.ok_or_else(|| M::Error::missing_field("cohort_target"))?;
                let return_period_target = return_period_target
                    .ok_or_else(|| M::Error::missing_field("return_period_target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ProductAnalyticsRetentionCellScope {
                    cohort_target,
                    return_period_target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionCellScopeVisitor)
    }
}
