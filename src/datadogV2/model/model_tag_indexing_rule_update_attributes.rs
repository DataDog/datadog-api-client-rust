// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a tag indexing rule. All fields are optional; omitted fields are unchanged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagIndexingRuleUpdateAttributes {
    /// When true, the rule excludes the listed tags and indexes all others.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Metric name prefixes excluded from the rule's scope.
    #[serde(rename = "ignored_metric_name_matches")]
    pub ignored_metric_name_matches: Option<Vec<String>>,
    /// Metric name prefixes (glob patterns) this rule applies to.
    #[serde(rename = "metric_name_matches")]
    pub metric_name_matches: Option<Vec<String>>,
    /// Human-readable name for the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Versioned configuration options for a tag indexing rule.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::TagIndexingRuleOptions>,
    /// Desired evaluation order. Returns 409 if the value conflicts with another rule; use POST /api/v2/metrics/tag-indexing-rules/order for atomic re-sequencing.
    #[serde(rename = "rule_order")]
    pub rule_order: Option<i64>,
    /// Tag keys managed by this rule.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagIndexingRuleUpdateAttributes {
    pub fn new() -> TagIndexingRuleUpdateAttributes {
        TagIndexingRuleUpdateAttributes {
            exclude_tags_mode: None,
            ignored_metric_name_matches: None,
            metric_name_matches: None,
            name: None,
            options: None,
            rule_order: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclude_tags_mode(mut self, value: bool) -> Self {
        self.exclude_tags_mode = Some(value);
        self
    }

    pub fn ignored_metric_name_matches(mut self, value: Vec<String>) -> Self {
        self.ignored_metric_name_matches = Some(value);
        self
    }

    pub fn metric_name_matches(mut self, value: Vec<String>) -> Self {
        self.metric_name_matches = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::TagIndexingRuleOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn rule_order(mut self, value: i64) -> Self {
        self.rule_order = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for TagIndexingRuleUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagIndexingRuleUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagIndexingRuleUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for TagIndexingRuleUpdateAttributesVisitor {
            type Value = TagIndexingRuleUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclude_tags_mode: Option<bool> = None;
                let mut ignored_metric_name_matches: Option<Vec<String>> = None;
                let mut metric_name_matches: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::TagIndexingRuleOptions> = None;
                let mut rule_order: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclude_tags_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_tags_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ignored_metric_name_matches" => {
                            if v.is_null() {
                                continue;
                            }
                            ignored_metric_name_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_name_matches" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_name_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_order" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TagIndexingRuleUpdateAttributes {
                    exclude_tags_mode,
                    ignored_metric_name_matches,
                    metric_name_matches,
                    name,
                    options,
                    rule_order,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagIndexingRuleUpdateAttributesVisitor)
    }
}
