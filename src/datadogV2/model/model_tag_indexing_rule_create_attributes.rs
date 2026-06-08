// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a tag indexing rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagIndexingRuleCreateAttributes {
    /// When true, the rule excludes the listed tags and indexes all others. When false (default), the rule includes only the listed tags.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Metric name prefixes excluded from the rule's scope.
    #[serde(rename = "ignored_metric_name_matches")]
    pub ignored_metric_name_matches: Option<Vec<String>>,
    /// Metric name prefixes (glob patterns) this rule applies to.
    #[serde(rename = "metric_name_matches")]
    pub metric_name_matches: Vec<String>,
    /// Human-readable name for the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Versioned configuration options for a tag indexing rule.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::TagIndexingRuleOptions>,
    /// Tag keys managed by this rule.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagIndexingRuleCreateAttributes {
    pub fn new(metric_name_matches: Vec<String>, name: String) -> TagIndexingRuleCreateAttributes {
        TagIndexingRuleCreateAttributes {
            exclude_tags_mode: None,
            ignored_metric_name_matches: None,
            metric_name_matches,
            name,
            options: None,
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

    pub fn options(mut self, value: crate::datadogV2::model::TagIndexingRuleOptions) -> Self {
        self.options = Some(value);
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

impl<'de> Deserialize<'de> for TagIndexingRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagIndexingRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for TagIndexingRuleCreateAttributesVisitor {
            type Value = TagIndexingRuleCreateAttributes;

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
                            metric_name_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let metric_name_matches = metric_name_matches
                    .ok_or_else(|| M::Error::missing_field("metric_name_matches"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = TagIndexingRuleCreateAttributes {
                    exclude_tags_mode,
                    ignored_metric_name_matches,
                    metric_name_matches,
                    name,
                    options,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagIndexingRuleCreateAttributesVisitor)
    }
}
