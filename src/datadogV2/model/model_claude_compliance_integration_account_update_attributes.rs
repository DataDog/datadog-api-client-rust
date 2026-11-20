// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes used to update a Claude Compliance integration account. Every field is optional. Only the provided fields are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClaudeComplianceIntegrationAccountUpdateAttributes {
    /// Authentication for updating the Claude Compliance integration account. Exactly one method is set.
    #[serde(rename = "authentication")]
    pub authentication:
        Option<crate::datadogV2::model::ClaudeComplianceIntegrationAccountAuthenticationUpdate>,
    /// Human-readable name of the Claude Compliance integration account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ClaudeComplianceIntegrationAccountUpdateAttributes {
    pub fn new() -> ClaudeComplianceIntegrationAccountUpdateAttributes {
        ClaudeComplianceIntegrationAccountUpdateAttributes {
            authentication: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn authentication(
        mut self,
        value: crate::datadogV2::model::ClaudeComplianceIntegrationAccountAuthenticationUpdate,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for ClaudeComplianceIntegrationAccountUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ClaudeComplianceIntegrationAccountUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClaudeComplianceIntegrationAccountUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for ClaudeComplianceIntegrationAccountUpdateAttributesVisitor {
            type Value = ClaudeComplianceIntegrationAccountUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<
                    crate::datadogV2::model::ClaudeComplianceIntegrationAccountAuthenticationUpdate,
                > = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::ClaudeComplianceIntegrationAccountAuthenticationUpdate::UnparsedObject(_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ClaudeComplianceIntegrationAccountUpdateAttributes {
                    authentication,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ClaudeComplianceIntegrationAccountUpdateAttributesVisitor)
    }
}
