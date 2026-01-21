// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration risks associated with the entity
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityEntityConfigRisks {
    /// Whether the entity has identity risks
    #[serde(rename = "hasIdentityRisk")]
    pub has_identity_risk: bool,
    /// Whether the entity has misconfigurations
    #[serde(rename = "hasMisconfiguration")]
    pub has_misconfiguration: bool,
    /// Whether the entity has privileged roles
    #[serde(rename = "hasPrivilegedRole")]
    pub has_privileged_role: bool,
    /// Whether the entity has privileged access
    #[serde(rename = "isPrivileged")]
    pub is_privileged: bool,
    /// Whether the entity is in a production environment
    #[serde(rename = "isProduction")]
    pub is_production: bool,
    /// Whether the entity is publicly accessible
    #[serde(rename = "isPubliclyAccessible")]
    pub is_publicly_accessible: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityEntityConfigRisks {
    pub fn new(
        has_identity_risk: bool,
        has_misconfiguration: bool,
        has_privileged_role: bool,
        is_privileged: bool,
        is_production: bool,
        is_publicly_accessible: bool,
    ) -> SecurityEntityConfigRisks {
        SecurityEntityConfigRisks {
            has_identity_risk,
            has_misconfiguration,
            has_privileged_role,
            is_privileged,
            is_production,
            is_publicly_accessible,
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

impl<'de> Deserialize<'de> for SecurityEntityConfigRisks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityEntityConfigRisksVisitor;
        impl<'a> Visitor<'a> for SecurityEntityConfigRisksVisitor {
            type Value = SecurityEntityConfigRisks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_identity_risk: Option<bool> = None;
                let mut has_misconfiguration: Option<bool> = None;
                let mut has_privileged_role: Option<bool> = None;
                let mut is_privileged: Option<bool> = None;
                let mut is_production: Option<bool> = None;
                let mut is_publicly_accessible: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hasIdentityRisk" => {
                            has_identity_risk =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hasMisconfiguration" => {
                            has_misconfiguration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hasPrivilegedRole" => {
                            has_privileged_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isPrivileged" => {
                            is_privileged =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isProduction" => {
                            is_production =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isPubliclyAccessible" => {
                            is_publicly_accessible =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let has_identity_risk = has_identity_risk
                    .ok_or_else(|| M::Error::missing_field("has_identity_risk"))?;
                let has_misconfiguration = has_misconfiguration
                    .ok_or_else(|| M::Error::missing_field("has_misconfiguration"))?;
                let has_privileged_role = has_privileged_role
                    .ok_or_else(|| M::Error::missing_field("has_privileged_role"))?;
                let is_privileged =
                    is_privileged.ok_or_else(|| M::Error::missing_field("is_privileged"))?;
                let is_production =
                    is_production.ok_or_else(|| M::Error::missing_field("is_production"))?;
                let is_publicly_accessible = is_publicly_accessible
                    .ok_or_else(|| M::Error::missing_field("is_publicly_accessible"))?;

                let content = SecurityEntityConfigRisks {
                    has_identity_risk,
                    has_misconfiguration,
                    has_privileged_role,
                    is_privileged,
                    is_production,
                    is_publicly_accessible,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityEntityConfigRisksVisitor)
    }
}
