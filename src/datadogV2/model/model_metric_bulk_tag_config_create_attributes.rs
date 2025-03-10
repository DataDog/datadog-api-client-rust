// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Optional parameters for bulk creating metric tag configurations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricBulkTagConfigCreateAttributes {
    /// A list of account emails to notify when the configuration is applied.
    #[serde(rename = "emails")]
    pub emails: Option<Vec<String>>,
    /// When set to true, the configuration will exclude the configured tags and include any other submitted tags.
    /// When set to false, the configuration will include the configured tags and exclude any other submitted tags.
    /// Defaults to false.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// When provided, all tags that have been actively queried are
    /// configured (and, therefore, remain queryable) for each metric that
    /// matches the given prefix.  Minimum value is 1 second, and maximum
    /// value is 7,776,000 seconds (90 days).
    #[serde(rename = "include_actively_queried_tags_window")]
    pub include_actively_queried_tags_window: Option<f64>,
    /// When set to true, the configuration overrides any existing
    /// configurations for the given metric with the new set of tags in this
    /// configuration request. If false, old configurations are kept and
    /// are merged with the set of tags in this configuration request.
    /// Defaults to true.
    #[serde(rename = "override_existing_configurations")]
    pub override_existing_configurations: Option<bool>,
    /// A list of tag names to apply to the configuration.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricBulkTagConfigCreateAttributes {
    pub fn new() -> MetricBulkTagConfigCreateAttributes {
        MetricBulkTagConfigCreateAttributes {
            emails: None,
            exclude_tags_mode: None,
            include_actively_queried_tags_window: None,
            override_existing_configurations: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn exclude_tags_mode(mut self, value: bool) -> Self {
        self.exclude_tags_mode = Some(value);
        self
    }

    pub fn include_actively_queried_tags_window(mut self, value: f64) -> Self {
        self.include_actively_queried_tags_window = Some(value);
        self
    }

    pub fn override_existing_configurations(mut self, value: bool) -> Self {
        self.override_existing_configurations = Some(value);
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

impl Default for MetricBulkTagConfigCreateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricBulkTagConfigCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricBulkTagConfigCreateAttributesVisitor;
        impl<'a> Visitor<'a> for MetricBulkTagConfigCreateAttributesVisitor {
            type Value = MetricBulkTagConfigCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut emails: Option<Vec<String>> = None;
                let mut exclude_tags_mode: Option<bool> = None;
                let mut include_actively_queried_tags_window: Option<f64> = None;
                let mut override_existing_configurations: Option<bool> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "emails" => {
                            if v.is_null() {
                                continue;
                            }
                            emails = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclude_tags_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_tags_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_actively_queried_tags_window" => {
                            if v.is_null() {
                                continue;
                            }
                            include_actively_queried_tags_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_existing_configurations" => {
                            if v.is_null() {
                                continue;
                            }
                            override_existing_configurations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = MetricBulkTagConfigCreateAttributes {
                    emails,
                    exclude_tags_mode,
                    include_actively_queried_tags_window,
                    override_existing_configurations,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricBulkTagConfigCreateAttributesVisitor)
    }
}
