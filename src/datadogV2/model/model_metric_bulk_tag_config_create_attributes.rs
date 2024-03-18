// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
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
    /// A list of tag names to apply to the configuration.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricBulkTagConfigCreateAttributes {
    pub fn new() -> MetricBulkTagConfigCreateAttributes {
        MetricBulkTagConfigCreateAttributes {
            emails: None,
            exclude_tags_mode: None,
            tags: None,
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

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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
                let mut tags: Option<Vec<String>> = None;
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
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricBulkTagConfigCreateAttributes {
                    emails,
                    exclude_tags_mode,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricBulkTagConfigCreateAttributesVisitor)
    }
}
