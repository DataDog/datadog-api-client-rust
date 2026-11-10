// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ArbitraryRuleResponseDataAttributesStrategy` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleResponseDataAttributesStrategy {
    /// The `strategy` `allocated_by`.
    #[serde(rename = "allocated_by")]
    pub allocated_by: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems>>,
    /// The `strategy` `allocated_by_filters`.
    #[serde(rename = "allocated_by_filters")]
    pub allocated_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByFiltersItems>>,
    /// The `strategy` `allocated_by_tag_keys`.
    #[serde(rename = "allocated_by_tag_keys")]
    pub allocated_by_tag_keys: Option<Vec<String>>,
    /// The `strategy` `based_on_costs`.
    #[serde(rename = "based_on_costs")]
    pub based_on_costs: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems>>,
    /// The rule `strategy` `based_on_timeseries`.
    #[serde(rename = "based_on_timeseries")]
    pub based_on_timeseries: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The `strategy` `evaluate_grouped_by_filters`.
    #[serde(rename = "evaluate_grouped_by_filters")]
    pub evaluate_grouped_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyEvaluateGroupedByFiltersItems>>,
    /// The `strategy` `evaluate_grouped_by_tag_keys`.
    #[serde(rename = "evaluate_grouped_by_tag_keys")]
    pub evaluate_grouped_by_tag_keys: Option<Vec<String>>,
    /// The `strategy` `granularity`.
    #[serde(rename = "granularity")]
    pub granularity: Option<String>,
    /// Only supported for `proportional_timeseries` and `even_timeseries` rules. Creates a synthetic tag from timeseries input rows before `tag_transforms` are applied.
    #[serde(rename = "mapping_transform")]
    pub mapping_transform: Option<crate::datadogV2::model::ArbitraryRuleMappingTransform>,
    /// The `strategy` `method`.
    #[serde(rename = "method")]
    pub method: String,
    /// Tags added to every cost row allocated by the rule.
    #[serde(rename = "static_tags")]
    pub static_tags: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsAllocatedTagsItems>>,
    /// Integration metrics query used to calculate coverage.
    #[serde(rename = "supporting_timeseries", default, with = "::serde_with::rust::double_option")]
    pub supporting_timeseries: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Only supported for `proportional_timeseries` and `even_timeseries` rules. Applies tag key renames and tag value transforms to timeseries input rows after `mapping_transform`.
    #[serde(rename = "tag_transforms")]
    pub tag_transforms: Option<crate::datadogV2::model::ArbitraryRuleTagTransforms>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ArbitraryRuleResponseDataAttributesStrategy {
    pub fn new(method: String) -> ArbitraryRuleResponseDataAttributesStrategy {
        ArbitraryRuleResponseDataAttributesStrategy {
            allocated_by: None,
            allocated_by_filters: None,
            allocated_by_tag_keys: None,
            based_on_costs: None,
            based_on_timeseries: None,
            evaluate_grouped_by_filters: None,
            evaluate_grouped_by_tag_keys: None,
            granularity: None,
            mapping_transform: None,
            method,
            static_tags: None,
            supporting_timeseries: None,
            tag_transforms: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allocated_by(
        mut self,
        value: Vec<
            crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems,
        >,
    ) -> Self {
        self.allocated_by = Some(value);
        self
    }

    pub fn allocated_by_filters(
        mut self,
        value: Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByFiltersItems>,
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
        value: Vec<
            crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems,
        >,
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
        value: Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyEvaluateGroupedByFiltersItems>,
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

    pub fn mapping_transform(
        mut self,
        value: crate::datadogV2::model::ArbitraryRuleMappingTransform,
    ) -> Self {
        self.mapping_transform = Some(value);
        self
    }

    pub fn static_tags(
        mut self,
        value: Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsAllocatedTagsItems>,
    ) -> Self {
        self.static_tags = Some(value);
        self
    }

    pub fn supporting_timeseries(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.supporting_timeseries = Some(value);
        self
    }

    pub fn tag_transforms(
        mut self,
        value: crate::datadogV2::model::ArbitraryRuleTagTransforms,
    ) -> Self {
        self.tag_transforms = Some(value);
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

impl<'de> Deserialize<'de> for ArbitraryRuleResponseDataAttributesStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleResponseDataAttributesStrategyVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleResponseDataAttributesStrategyVisitor {
            type Value = ArbitraryRuleResponseDataAttributesStrategy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocated_by: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems>> = None;
                let mut allocated_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByFiltersItems>> = None;
                let mut allocated_by_tag_keys: Option<Vec<String>> = None;
                let mut based_on_costs: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems>> = None;
                let mut based_on_timeseries: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut evaluate_grouped_by_filters: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyEvaluateGroupedByFiltersItems>> = None;
                let mut evaluate_grouped_by_tag_keys: Option<Vec<String>> = None;
                let mut granularity: Option<String> = None;
                let mut mapping_transform: Option<
                    crate::datadogV2::model::ArbitraryRuleMappingTransform,
                > = None;
                let mut method: Option<String> = None;
                let mut static_tags: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsAllocatedTagsItems>> = None;
                let mut supporting_timeseries: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut tag_transforms: Option<
                    crate::datadogV2::model::ArbitraryRuleTagTransforms,
                > = None;
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
                        "mapping_transform" => {
                            if v.is_null() {
                                continue;
                            }
                            mapping_transform =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "method" => {
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "static_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            static_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "supporting_timeseries" => {
                            supporting_timeseries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_transforms" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_transforms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let method = method.ok_or_else(|| M::Error::missing_field("method"))?;

                let content = ArbitraryRuleResponseDataAttributesStrategy {
                    allocated_by,
                    allocated_by_filters,
                    allocated_by_tag_keys,
                    based_on_costs,
                    based_on_timeseries,
                    evaluate_grouped_by_filters,
                    evaluate_grouped_by_tag_keys,
                    granularity,
                    mapping_transform,
                    method,
                    static_tags,
                    supporting_timeseries,
                    tag_transforms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryRuleResponseDataAttributesStrategyVisitor)
    }
}
