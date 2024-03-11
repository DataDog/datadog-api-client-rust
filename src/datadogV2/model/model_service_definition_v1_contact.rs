// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Contact information about the service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV1Contact {
    /// Service owner’s email.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Service owner’s Slack channel.
    #[serde(rename = "slack")]
    pub slack: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV1Contact {
    pub fn new() -> ServiceDefinitionV1Contact {
        ServiceDefinitionV1Contact {
            email: None,
            slack: None,
            _unparsed: false,
        }
    }

    pub fn email(&mut self, value: String) -> &mut Self {
        self.email = Some(value);
        self
    }

    pub fn slack(&mut self, value: String) -> &mut Self {
        self.slack = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV1Contact {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV1Contact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV1ContactVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV1ContactVisitor {
            type Value = ServiceDefinitionV1Contact;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut slack: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slack" => {
                            if v.is_null() {
                                continue;
                            }
                            slack = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionV1Contact {
                    email,
                    slack,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV1ContactVisitor)
    }
}
