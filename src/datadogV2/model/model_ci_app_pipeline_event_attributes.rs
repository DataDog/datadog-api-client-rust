// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON object containing all event attributes and their associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelineEventAttributes {
    /// JSON object of attributes from CI Visibility pipeline events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Pipeline execution level.
    #[serde(rename = "ci_level")]
    pub ci_level: Option<crate::datadogV2::model::CIAppPipelineLevel>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelineEventAttributes {
    pub fn new() -> CIAppPipelineEventAttributes {
        CIAppPipelineEventAttributes {
            attributes: None,
            ci_level: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn ci_level(mut self, value: crate::datadogV2::model::CIAppPipelineLevel) -> Self {
        self.ci_level = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for CIAppPipelineEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppPipelineEventAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelineEventAttributesVisitor;
        impl<'a> Visitor<'a> for CIAppPipelineEventAttributesVisitor {
            type Value = CIAppPipelineEventAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut ci_level: Option<crate::datadogV2::model::CIAppPipelineLevel> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_level" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _ci_level) = ci_level {
                                match _ci_level {
                                    crate::datadogV2::model::CIAppPipelineLevel::UnparsedObject(
                                        _ci_level,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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

                let content = CIAppPipelineEventAttributes {
                    attributes,
                    ci_level,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelineEventAttributesVisitor)
    }
}
