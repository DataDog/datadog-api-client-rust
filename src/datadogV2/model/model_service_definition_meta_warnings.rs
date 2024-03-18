// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema validation warnings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionMetaWarnings {
    /// The warning instance location.
    #[serde(rename = "instance-location")]
    pub instance_location: Option<String>,
    /// The warning keyword location.
    #[serde(rename = "keyword-location")]
    pub keyword_location: Option<String>,
    /// The warning message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionMetaWarnings {
    pub fn new() -> ServiceDefinitionMetaWarnings {
        ServiceDefinitionMetaWarnings {
            instance_location: None,
            keyword_location: None,
            message: None,
            _unparsed: false,
        }
    }

    pub fn instance_location(mut self, value: String) -> Self {
        self.instance_location = Some(value);
        self
    }

    pub fn keyword_location(mut self, value: String) -> Self {
        self.keyword_location = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}

impl Default for ServiceDefinitionMetaWarnings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionMetaWarnings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionMetaWarningsVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionMetaWarningsVisitor {
            type Value = ServiceDefinitionMetaWarnings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut instance_location: Option<String> = None;
                let mut keyword_location: Option<String> = None;
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "instance-location" => {
                            if v.is_null() {
                                continue;
                            }
                            instance_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "keyword-location" => {
                            if v.is_null() {
                                continue;
                            }
                            keyword_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionMetaWarnings {
                    instance_location,
                    keyword_location,
                    message,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionMetaWarningsVisitor)
    }
}
