// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an integration account. The `integration` field is a strongly-typed, per-integration union.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationAccountAttributes {
    /// Strongly-typed, per-integration configuration. Exactly one integration variant is set, selected by its `type`.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::IntegrationAccountIntegration,
    /// Human-readable name of the account.
    #[serde(rename = "name")]
    pub name: String,
    /// Read-only permission information for the account, derived from its restriction policy.
    #[serde(rename = "permissions")]
    pub permissions: Option<crate::datadogV2::model::IntegrationAccountPermissions>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAccountAttributes {
    pub fn new(
        integration: crate::datadogV2::model::IntegrationAccountIntegration,
        name: String,
    ) -> IntegrationAccountAttributes {
        IntegrationAccountAttributes {
            integration,
            name,
            permissions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn permissions(
        mut self,
        value: crate::datadogV2::model::IntegrationAccountPermissions,
    ) -> Self {
        self.permissions = Some(value);
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

impl<'de> Deserialize<'de> for IntegrationAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAccountAttributesVisitor;
        impl<'a> Visitor<'a> for IntegrationAccountAttributesVisitor {
            type Value = IntegrationAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut integration: Option<
                    crate::datadogV2::model::IntegrationAccountIntegration,
                > = None;
                let mut name: Option<String> = None;
                let mut permissions: Option<
                    crate::datadogV2::model::IntegrationAccountPermissions,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration) = integration {
                                match _integration {
                                    crate::datadogV2::model::IntegrationAccountIntegration::UnparsedObject(_integration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "permissions" => {
                            if v.is_null() {
                                continue;
                            }
                            permissions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = IntegrationAccountAttributes {
                    integration,
                    name,
                    permissions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAccountAttributesVisitor)
    }
}
