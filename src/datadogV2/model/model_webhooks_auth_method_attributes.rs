// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a webhooks auth method.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WebhooksAuthMethodAttributes {
    /// Authentication protocol used by the auth method.
    #[serde(rename = "protocol")]
    pub protocol: Option<crate::datadogV2::model::WebhooksAuthMethodProtocol>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebhooksAuthMethodAttributes {
    pub fn new() -> WebhooksAuthMethodAttributes {
        WebhooksAuthMethodAttributes {
            protocol: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn protocol(mut self, value: crate::datadogV2::model::WebhooksAuthMethodProtocol) -> Self {
        self.protocol = Some(value);
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

impl Default for WebhooksAuthMethodAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WebhooksAuthMethodAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhooksAuthMethodAttributesVisitor;
        impl<'a> Visitor<'a> for WebhooksAuthMethodAttributesVisitor {
            type Value = WebhooksAuthMethodAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut protocol: Option<crate::datadogV2::model::WebhooksAuthMethodProtocol> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "protocol" => {
                            if v.is_null() {
                                continue;
                            }
                            protocol = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _protocol) = protocol {
                                match _protocol {
                                    crate::datadogV2::model::WebhooksAuthMethodProtocol::UnparsedObject(_protocol) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = WebhooksAuthMethodAttributes {
                    protocol,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebhooksAuthMethodAttributesVisitor)
    }
}
