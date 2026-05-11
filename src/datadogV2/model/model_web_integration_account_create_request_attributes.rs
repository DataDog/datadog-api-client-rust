// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for creating a web integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WebIntegrationAccountCreateRequestAttributes {
    /// A human-readable name for the account. Must be unique among accounts of the same integration.
    #[serde(rename = "name")]
    pub name: String,
    /// Integration-specific secrets. The shape of this object varies by integration. Secrets
    /// are write-only and never returned by the API.
    #[serde(rename = "secrets")]
    pub secrets: std::collections::BTreeMap<String, serde_json::Value>,
    /// Integration-specific settings. The shape of this object varies by integration.
    #[serde(rename = "settings")]
    pub settings: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebIntegrationAccountCreateRequestAttributes {
    pub fn new(
        name: String,
        secrets: std::collections::BTreeMap<String, serde_json::Value>,
        settings: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> WebIntegrationAccountCreateRequestAttributes {
        WebIntegrationAccountCreateRequestAttributes {
            name,
            secrets,
            settings,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for WebIntegrationAccountCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebIntegrationAccountCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for WebIntegrationAccountCreateRequestAttributesVisitor {
            type Value = WebIntegrationAccountCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut secrets: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut settings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secrets" => {
                            secrets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let secrets = secrets.ok_or_else(|| M::Error::missing_field("secrets"))?;
                let settings = settings.ok_or_else(|| M::Error::missing_field("settings"))?;

                let content = WebIntegrationAccountCreateRequestAttributes {
                    name,
                    secrets,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebIntegrationAccountCreateRequestAttributesVisitor)
    }
}
