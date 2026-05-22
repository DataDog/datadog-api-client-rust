// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The organization ID, integration identifier, and integration-specific configuration payload for an entity integration configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityIntegrationConfigAttributes {
    /// Integration-specific configuration payload. The shape of this object depends on the integration identified by the path parameter. For `github`, the object must contain an `enabled_repos` array. For `jira`, it must contain an `enabled_projects` array. For `pagerduty`, it must contain an `accounts` array.
    #[serde(rename = "config")]
    pub config: std::collections::BTreeMap<String, serde_json::Value>,
    /// The identifier of the integration this configuration applies to (for example, `github`, `jira`, or `pagerduty`).
    #[serde(rename = "integration_id")]
    pub integration_id: String,
    /// The Datadog organization identifier that owns this configuration.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityIntegrationConfigAttributes {
    pub fn new(
        config: std::collections::BTreeMap<String, serde_json::Value>,
        integration_id: String,
        org_id: i64,
    ) -> EntityIntegrationConfigAttributes {
        EntityIntegrationConfigAttributes {
            config,
            integration_id,
            org_id,
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

impl<'de> Deserialize<'de> for EntityIntegrationConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityIntegrationConfigAttributesVisitor;
        impl<'a> Visitor<'a> for EntityIntegrationConfigAttributesVisitor {
            type Value = EntityIntegrationConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut integration_id: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_id" => {
                            integration_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config = config.ok_or_else(|| M::Error::missing_field("config"))?;
                let integration_id =
                    integration_id.ok_or_else(|| M::Error::missing_field("integration_id"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;

                let content = EntityIntegrationConfigAttributes {
                    config,
                    integration_id,
                    org_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityIntegrationConfigAttributesVisitor)
    }
}
