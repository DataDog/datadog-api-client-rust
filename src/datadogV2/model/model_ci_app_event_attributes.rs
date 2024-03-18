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
pub struct CIAppEventAttributes {
    /// JSON object of attributes from CI Visibility test events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Test run level.
    #[serde(rename = "test_level")]
    pub test_level: Option<crate::datadogV2::model::CIAppTestLevel>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppEventAttributes {
    pub fn new() -> CIAppEventAttributes {
        CIAppEventAttributes {
            attributes: None,
            tags: None,
            test_level: None,
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

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn test_level(mut self, value: crate::datadogV2::model::CIAppTestLevel) -> Self {
        self.test_level = Some(value);
        self
    }
}

impl Default for CIAppEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppEventAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppEventAttributesVisitor;
        impl<'a> Visitor<'a> for CIAppEventAttributesVisitor {
            type Value = CIAppEventAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut test_level: Option<crate::datadogV2::model::CIAppTestLevel> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_level" => {
                            if v.is_null() {
                                continue;
                            }
                            test_level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _test_level) = test_level {
                                match _test_level {
                                    crate::datadogV2::model::CIAppTestLevel::UnparsedObject(
                                        _test_level,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = CIAppEventAttributes {
                    attributes,
                    tags,
                    test_level,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppEventAttributesVisitor)
    }
}
