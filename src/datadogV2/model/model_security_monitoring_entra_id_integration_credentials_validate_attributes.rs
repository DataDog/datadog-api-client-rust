// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Entra ID credentials to validate against the external entity source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes {
    /// The domain associated with the external entity source.
    #[serde(rename = "domain")]
    pub domain: String,
    /// The source type for an Entra ID entity context sync.
    #[serde(rename = "integration_type")]
    pub integration_type: crate::datadogV2::model::SecurityMonitoringIntegrationTypeEntraId,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes {
    pub fn new(
        domain: String,
        integration_type: crate::datadogV2::model::SecurityMonitoringIntegrationTypeEntraId,
    ) -> SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes {
        SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes {
            domain,
            integration_type,
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

impl<'de> Deserialize<'de> for SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributesVisitor {
            type Value = SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain: Option<String> = None;
                let mut integration_type: Option<
                    crate::datadogV2::model::SecurityMonitoringIntegrationTypeEntraId,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domain" => {
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration_type) = integration_type {
                                match _integration_type {
                                    crate::datadogV2::model::SecurityMonitoringIntegrationTypeEntraId::UnparsedObject(_integration_type) => {
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
                let domain = domain.ok_or_else(|| M::Error::missing_field("domain"))?;
                let integration_type =
                    integration_type.ok_or_else(|| M::Error::missing_field("integration_type"))?;

                let content = SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes {
                    domain,
                    integration_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributesVisitor,
        )
    }
}
