// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a tag indexing rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagIndexingRuleAttributes {
    /// Timestamp when the rule was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Handle of the user who created the rule.
    #[serde(rename = "created_by_handle")]
    pub created_by_handle: Option<String>,
    /// When true, the rule excludes the listed tags and indexes all others. When false (default), the rule includes only the listed tags.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Metric name prefixes excluded from the rule's scope.
    #[serde(rename = "ignored_metric_name_matches")]
    pub ignored_metric_name_matches: Option<Vec<String>>,
    /// Metric name prefixes (glob patterns) this rule applies to.
    #[serde(rename = "metric_name_matches")]
    pub metric_name_matches: Option<Vec<String>>,
    /// Timestamp when the rule was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Handle of the user who last modified the rule.
    #[serde(rename = "modified_by_handle")]
    pub modified_by_handle: Option<String>,
    /// Human-readable name for the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Versioned configuration options for a tag indexing rule.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::TagIndexingRuleOptions>,
    /// Evaluation order within the org. Lower values are evaluated first. Assigned server-side on create (max+1); pass on update to change the rule's position.
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

impl TagIndexingRuleAttributes {
    pub fn new() -> TagIndexingRuleAttributes {
        TagIndexingRuleAttributes {
            created_at: None,
            created_by_handle: None,
            exclude_tags_mode: None,
            ignored_metric_name_matches: None,
            metric_name_matches: None,
            modified_at: None,
            modified_by_handle: None,
            name: None,
            options: None,
            rule_order: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_handle(mut self, value: String) -> Self {
        self.created_by_handle = Some(value);
        self
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

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn modified_by_handle(mut self, value: String) -> Self {
        self.modified_by_handle = Some(value);
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

impl Default for TagIndexingRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagIndexingRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagIndexingRuleAttributesVisitor;
        impl<'a> Visitor<'a> for TagIndexingRuleAttributesVisitor {
            type Value = TagIndexingRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by_handle: Option<String> = None;
                let mut exclude_tags_mode: Option<bool> = None;
                let mut ignored_metric_name_matches: Option<Vec<String>> = None;
                let mut metric_name_matches: Option<Vec<String>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by_handle: Option<String> = None;
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
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by_handle =
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

                let content = TagIndexingRuleAttributes {
                    created_at,
                    created_by_handle,
                    exclude_tags_mode,
                    ignored_metric_name_matches,
                    metric_name_matches,
                    modified_at,
                    modified_by_handle,
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

        deserializer.deserialize_any(TagIndexingRuleAttributesVisitor)
    }
}
