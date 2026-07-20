// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data payload for tag indexing rule options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagIndexingRuleOptionsData {
    /// Options for dynamic tag indexing applied per metric, such as tags filtered by query usage.
    ///
    /// Before a tag key is dropped by this rule, two grace period conditions must be met:
    ///
    /// 1. The metric must be submitted for at least as long as the selected window.
    /// 2. A tag key must have been submitted for at least 15 days.
    ///
    /// Any metric or tag key that does not meet these conditions are excluded from this
    /// indexing rule. The `exclude_not_*` fields require `exclude_tags_mode` to be set to `true`.
    #[serde(rename = "dynamic_tags")]
    pub dynamic_tags: Option<crate::datadogV2::model::TagIndexingRuleDynamicTags>,
    /// When true, the rule applies to metrics that were ingested before the rule was created.
    #[serde(rename = "manage_preexisting_metrics")]
    pub manage_preexisting_metrics: Option<bool>,
    /// Criteria for matching metrics based on query state.
    #[serde(rename = "metric_match")]
    pub metric_match: Option<crate::datadogV2::model::TagIndexingRuleMetricMatch>,
    /// When true, this rule's tag list overrides tags configured by earlier rules for the same metric. When false (default), tags from all matching rules are combined.
    #[serde(rename = "override_previous_rules")]
    pub override_previous_rules: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagIndexingRuleOptionsData {
    pub fn new() -> TagIndexingRuleOptionsData {
        TagIndexingRuleOptionsData {
            dynamic_tags: None,
            manage_preexisting_metrics: None,
            metric_match: None,
            override_previous_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dynamic_tags(
        mut self,
        value: crate::datadogV2::model::TagIndexingRuleDynamicTags,
    ) -> Self {
        self.dynamic_tags = Some(value);
        self
    }

    pub fn manage_preexisting_metrics(mut self, value: bool) -> Self {
        self.manage_preexisting_metrics = Some(value);
        self
    }

    pub fn metric_match(
        mut self,
        value: crate::datadogV2::model::TagIndexingRuleMetricMatch,
    ) -> Self {
        self.metric_match = Some(value);
        self
    }

    pub fn override_previous_rules(mut self, value: bool) -> Self {
        self.override_previous_rules = Some(value);
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

impl Default for TagIndexingRuleOptionsData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagIndexingRuleOptionsData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagIndexingRuleOptionsDataVisitor;
        impl<'a> Visitor<'a> for TagIndexingRuleOptionsDataVisitor {
            type Value = TagIndexingRuleOptionsData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dynamic_tags: Option<crate::datadogV2::model::TagIndexingRuleDynamicTags> =
                    None;
                let mut manage_preexisting_metrics: Option<bool> = None;
                let mut metric_match: Option<crate::datadogV2::model::TagIndexingRuleMetricMatch> =
                    None;
                let mut override_previous_rules: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dynamic_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            dynamic_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "manage_preexisting_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            manage_preexisting_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_match" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_match =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_previous_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            override_previous_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TagIndexingRuleOptionsData {
                    dynamic_tags,
                    manage_preexisting_metrics,
                    metric_match,
                    override_previous_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagIndexingRuleOptionsDataVisitor)
    }
}
