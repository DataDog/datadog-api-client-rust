// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an incident user-defined role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserDefinedRolePatchDataAttributesRequest {
    /// A description of the user-defined role.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The name of the user-defined role.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Policy configuration for a user-defined role.
    #[serde(rename = "policy")]
    pub policy: Option<crate::datadogV2::model::IncidentUserDefinedRolePolicy>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserDefinedRolePatchDataAttributesRequest {
    pub fn new() -> IncidentUserDefinedRolePatchDataAttributesRequest {
        IncidentUserDefinedRolePatchDataAttributesRequest {
            description: None,
            name: None,
            policy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn policy(mut self, value: crate::datadogV2::model::IncidentUserDefinedRolePolicy) -> Self {
        self.policy = Some(value);
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

impl Default for IncidentUserDefinedRolePatchDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentUserDefinedRolePatchDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserDefinedRolePatchDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentUserDefinedRolePatchDataAttributesRequestVisitor {
            type Value = IncidentUserDefinedRolePatchDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut policy: Option<crate::datadogV2::model::IncidentUserDefinedRolePolicy> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy" => {
                            if v.is_null() {
                                continue;
                            }
                            policy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentUserDefinedRolePatchDataAttributesRequest {
                    description,
                    name,
                    policy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserDefinedRolePatchDataAttributesRequestVisitor)
    }
}
