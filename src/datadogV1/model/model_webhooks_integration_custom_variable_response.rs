// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom variable for Webhook integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WebhooksIntegrationCustomVariableResponse {
    /// Make custom variable is secret or not.
    /// If the custom variable is secret, the value is not returned in the response payload.
    #[serde(rename = "is_secret")]
    pub is_secret: bool,
    /// The name of the variable. It corresponds with `<CUSTOM_VARIABLE_NAME>`. It must only contains upper-case characters, integers or underscores.
    #[serde(rename = "name")]
    pub name: String,
    /// Value of the custom variable. It won't be returned if the variable is secret.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebhooksIntegrationCustomVariableResponse {
    pub fn new(is_secret: bool, name: String) -> WebhooksIntegrationCustomVariableResponse {
        WebhooksIntegrationCustomVariableResponse {
            is_secret,
            name,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
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

impl<'de> Deserialize<'de> for WebhooksIntegrationCustomVariableResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhooksIntegrationCustomVariableResponseVisitor;
        impl<'a> Visitor<'a> for WebhooksIntegrationCustomVariableResponseVisitor {
            type Value = WebhooksIntegrationCustomVariableResponse;

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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_secret" => {
                            is_secret = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_secret = is_secret.ok_or_else(|| M::Error::missing_field("is_secret"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = WebhooksIntegrationCustomVariableResponse {
                    is_secret,
                    name,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebhooksIntegrationCustomVariableResponseVisitor)
    }
}
