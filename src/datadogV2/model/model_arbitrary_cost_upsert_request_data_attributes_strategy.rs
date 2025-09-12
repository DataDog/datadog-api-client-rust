// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ArbitraryCostUpsertRequestDataAttributesStrategy` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryCostUpsertRequestDataAttributesStrategy {
    /// The `strategy` `allocated_by`.
    #[serde(rename = "allocated_by")]
    pub allocated_by: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyAllocatedByItems>>,
    /// The `strategy` `allocated_by_filters`.
    #[serde(rename = "allocated_by_filters")]
    pub allocated_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyAllocatedByFiltersItems>>,
    /// The `strategy` `allocated_by_tag_keys`.
    #[serde(rename = "allocated_by_tag_keys")]
    pub allocated_by_tag_keys: Option<Vec<String>>,
    /// The `strategy` `based_on_costs`.
    #[serde(rename = "based_on_costs")]
    pub based_on_costs: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyBasedOnCostsItems>>,
    /// The `strategy` `based_on_timeseries`.
    #[serde(rename = "based_on_timeseries")]
    pub based_on_timeseries: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The `strategy` `evaluate_grouped_by_filters`.
    #[serde(rename = "evaluate_grouped_by_filters")]
    pub evaluate_grouped_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyEvaluateGroupedByFiltersItems>>,
    /// The `strategy` `evaluate_grouped_by_tag_keys`.
    #[serde(rename = "evaluate_grouped_by_tag_keys")]
    pub evaluate_grouped_by_tag_keys: Option<Vec<String>>,
    /// The `strategy` `granularity`.
    #[serde(rename = "granularity")]
    pub granularity: Option<String>,
    /// The `strategy` `method`.
    #[serde(rename = "method")]
    pub method: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ArbitraryCostUpsertRequestDataAttributesStrategy {
    pub fn new(method: String) -> ArbitraryCostUpsertRequestDataAttributesStrategy {
        ArbitraryCostUpsertRequestDataAttributesStrategy {
            allocated_by: None,
            allocated_by_filters: None,
            allocated_by_tag_keys: None,
            based_on_costs: None,
            based_on_timeseries: None,
            evaluate_grouped_by_filters: None,
            evaluate_grouped_by_tag_keys: None,
            granularity: None,
            method,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allocated_by(
        mut self,
        value: Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyAllocatedByItems>,
    ) -> Self {
        self.allocated_by = Some(value);
        self
    }

    pub fn allocated_by_filters(
        mut self,
        value: Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyAllocatedByFiltersItems>,
    ) -> Self {
        self.allocated_by_filters = Some(value);
        self
    }

    pub fn allocated_by_tag_keys(mut self, value: Vec<String>) -> Self {
        self.allocated_by_tag_keys = Some(value);
        self
    }

    pub fn based_on_costs(
        mut self,
        value: Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyBasedOnCostsItems>,
    ) -> Self {
        self.based_on_costs = Some(value);
        self
    }

    pub fn based_on_timeseries(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.based_on_timeseries = Some(value);
        self
    }

    pub fn evaluate_grouped_by_filters(
        mut self,
        value: Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyEvaluateGroupedByFiltersItems>,
    ) -> Self {
        self.evaluate_grouped_by_filters = Some(value);
        self
    }

    pub fn evaluate_grouped_by_tag_keys(mut self, value: Vec<String>) -> Self {
        self.evaluate_grouped_by_tag_keys = Some(value);
        self
    }

    pub fn granularity(mut self, value: String) -> Self {
        self.granularity = Some(value);
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

impl<'de> Deserialize<'de> for ArbitraryCostUpsertRequestDataAttributesStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryCostUpsertRequestDataAttributesStrategyVisitor;
        impl<'a> Visitor<'a> for ArbitraryCostUpsertRequestDataAttributesStrategyVisitor {
            type Value = ArbitraryCostUpsertRequestDataAttributesStrategy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocated_by: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyAllocatedByItems>> = None;
                let mut allocated_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyAllocatedByFiltersItems>> = None;
                let mut allocated_by_tag_keys: Option<Vec<String>> = None;
                let mut based_on_costs: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyBasedOnCostsItems>> = None;
                let mut based_on_timeseries: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut evaluate_grouped_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyEvaluateGroupedByFiltersItems>> = None;
                let mut evaluate_grouped_by_tag_keys: Option<Vec<String>> = None;
                let mut granularity: Option<String> = None;
                let mut method: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allocated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            allocated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allocated_by_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            allocated_by_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allocated_by_tag_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            allocated_by_tag_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "based_on_costs" => {
                            if v.is_null() {
                                continue;
                            }
                            based_on_costs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "based_on_timeseries" => {
                            if v.is_null() {
                                continue;
                            }
                            based_on_timeseries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluate_grouped_by_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluate_grouped_by_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluate_grouped_by_tag_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluate_grouped_by_tag_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "granularity" => {
                            if v.is_null() {
                                continue;
                            }
                            granularity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "method" => {
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let method = method.ok_or_else(|| M::Error::missing_field("method"))?;

                let content = ArbitraryCostUpsertRequestDataAttributesStrategy {
                    allocated_by,
                    allocated_by_filters,
                    allocated_by_tag_keys,
                    based_on_costs,
                    based_on_timeseries,
                    evaluate_grouped_by_filters,
                    evaluate_grouped_by_tag_keys,
                    granularity,
                    method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryCostUpsertRequestDataAttributesStrategyVisitor)
    }
}
