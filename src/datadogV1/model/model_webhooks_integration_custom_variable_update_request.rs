// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update request of a custom variable object.
///
/// *All properties are optional.*
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WebhooksIntegrationCustomVariableUpdateRequest {
    /// Make custom variable is secret or not.
    /// If the custom variable is secret, the value is not returned in the response payload.
    #[serde(rename = "is_secret")]
    pub is_secret: Option<bool>,
    /// The name of the variable. It corresponds with `<CUSTOM_VARIABLE_NAME>`. It must only contains upper-case characters, integers or underscores.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Value of the custom variable.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebhooksIntegrationCustomVariableUpdateRequest {
    pub fn new() -> WebhooksIntegrationCustomVariableUpdateRequest {
        WebhooksIntegrationCustomVariableUpdateRequest {
            is_secret: None,
            name: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for WebhooksIntegrationCustomVariableUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WebhooksIntegrationCustomVariableUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhooksIntegrationCustomVariableUpdateRequestVisitor;
        impl<'a> Visitor<'a> for WebhooksIntegrationCustomVariableUpdateRequestVisitor {
            type Value = WebhooksIntegrationCustomVariableUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_secret: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut value: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_secret" => {
                            if v.is_null() {
                                continue;
                            }
                            is_secret = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WebhooksIntegrationCustomVariableUpdateRequest {
                    is_secret,
                    name,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebhooksIntegrationCustomVariableUpdateRequestVisitor)
    }
}
