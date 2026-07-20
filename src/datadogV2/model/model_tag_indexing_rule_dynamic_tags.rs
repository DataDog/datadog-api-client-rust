// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options for dynamic tag indexing applied per metric, such as tags filtered by query usage.
///
/// Before a tag key is dropped by this rule, two grace period conditions must be met:
///
/// 1. The metric must be submitted for at least as long as the selected window.
/// 2. A tag key must have been submitted for at least 15 days.
///
/// Any metric or tag key that does not meet these conditions are excluded from this
/// indexing rule. The `exclude_not_*` fields require `exclude_tags_mode` to be set to `true`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagIndexingRuleDynamicTags {
    /// Tags that have not been queried within this window are excluded from indexing. Maximum of `7776000` (90 days).
    #[serde(rename = "exclude_not_queried_window_seconds")]
    pub exclude_not_queried_window_seconds: Option<i64>,
    /// Tags not used in any dashboards,  monitors, notebooks, or SLOs are excluded from indexing.
    #[serde(rename = "exclude_not_used_in_assets")]
    pub exclude_not_used_in_assets: Option<bool>,
    /// Window in seconds for evaluating queried tags.
    #[serde(rename = "queried_tags_window_seconds")]
    pub queried_tags_window_seconds: Option<i64>,
    /// When true, tags from related assets are included.
    #[serde(rename = "related_asset_tags")]
    pub related_asset_tags: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagIndexingRuleDynamicTags {
    pub fn new() -> TagIndexingRuleDynamicTags {
        TagIndexingRuleDynamicTags {
            exclude_not_queried_window_seconds: None,
            exclude_not_used_in_assets: None,
            queried_tags_window_seconds: None,
            related_asset_tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclude_not_queried_window_seconds(mut self, value: i64) -> Self {
        self.exclude_not_queried_window_seconds = Some(value);
        self
    }

    pub fn exclude_not_used_in_assets(mut self, value: bool) -> Self {
        self.exclude_not_used_in_assets = Some(value);
        self
    }

    pub fn queried_tags_window_seconds(mut self, value: i64) -> Self {
        self.queried_tags_window_seconds = Some(value);
        self
    }

    pub fn related_asset_tags(mut self, value: bool) -> Self {
        self.related_asset_tags = Some(value);
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

impl Default for TagIndexingRuleDynamicTags {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagIndexingRuleDynamicTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagIndexingRuleDynamicTagsVisitor;
        impl<'a> Visitor<'a> for TagIndexingRuleDynamicTagsVisitor {
            type Value = TagIndexingRuleDynamicTags;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclude_not_queried_window_seconds: Option<i64> = None;
                let mut exclude_not_used_in_assets: Option<bool> = None;
                let mut queried_tags_window_seconds: Option<i64> = None;
                let mut related_asset_tags: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclude_not_queried_window_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_not_queried_window_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclude_not_used_in_assets" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_not_used_in_assets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queried_tags_window_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            queried_tags_window_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "related_asset_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            related_asset_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TagIndexingRuleDynamicTags {
                    exclude_not_queried_window_seconds,
                    exclude_not_used_in_assets,
                    queried_tags_window_seconds,
                    related_asset_tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagIndexingRuleDynamicTagsVisitor)
    }
}
