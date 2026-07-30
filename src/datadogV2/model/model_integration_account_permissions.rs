// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Read-only permission information for the account, derived from its restriction policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationAccountPermissions {
    /// Restriction-policy resource identifier of this account.
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    /// Whether the requesting user may edit this account.
    #[serde(rename = "user_can_edit")]
    pub user_can_edit: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAccountPermissions {
    pub fn new() -> IntegrationAccountPermissions {
        IntegrationAccountPermissions {
            resource_id: None,
            user_can_edit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    pub fn user_can_edit(mut self, value: bool) -> Self {
        self.user_can_edit = Some(value);
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

impl Default for IntegrationAccountPermissions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationAccountPermissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAccountPermissionsVisitor;
        impl<'a> Visitor<'a> for IntegrationAccountPermissionsVisitor {
            type Value = IntegrationAccountPermissions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut resource_id: Option<String> = None;
                let mut user_can_edit: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "resource_id" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_can_edit" => {
                            if v.is_null() {
                                continue;
                            }
                            user_can_edit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationAccountPermissions {
                    resource_id,
                    user_can_edit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAccountPermissionsVisitor)
    }
}
