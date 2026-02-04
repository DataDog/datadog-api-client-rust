// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a connection group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConnectionGroupDataAttributesRequest {
    /// List of connection IDs associated with the connection group.
    #[serde(rename = "connections")]
    pub connections: Option<Vec<String>>,
    /// The description of the connection group.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The integration type of the connection group.
    #[serde(rename = "integration_type")]
    pub integration_type: Option<String>,
    /// The name of the connection group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Policy attributes for the connection group.
    #[serde(rename = "policy_attributes")]
    pub policy_attributes: Option<String>,
    /// Tag keys associated with the connection group.
    #[serde(rename = "tag_keys")]
    pub tag_keys: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConnectionGroupDataAttributesRequest {
    pub fn new() -> ConnectionGroupDataAttributesRequest {
        ConnectionGroupDataAttributesRequest {
            connections: None,
            description: None,
            integration_type: None,
            name: None,
            policy_attributes: None,
            tag_keys: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn connections(mut self, value: Vec<String>) -> Self {
        self.connections = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn integration_type(mut self, value: String) -> Self {
        self.integration_type = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn policy_attributes(mut self, value: String) -> Self {
        self.policy_attributes = Some(value);
        self
    }

    pub fn tag_keys(mut self, value: Vec<String>) -> Self {
        self.tag_keys = Some(value);
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

impl Default for ConnectionGroupDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ConnectionGroupDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConnectionGroupDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for ConnectionGroupDataAttributesRequestVisitor {
            type Value = ConnectionGroupDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connections: Option<Vec<String>> = None;
                let mut description: Option<String> = None;
                let mut integration_type: Option<String> = None;
                let mut name: Option<String> = None;
                let mut policy_attributes: Option<String> = None;
                let mut tag_keys: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connections" => {
                            if v.is_null() {
                                continue;
                            }
                            connections =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_attributes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ConnectionGroupDataAttributesRequest {
                    connections,
                    description,
                    integration_type,
                    name,
                    policy_attributes,
                    tag_keys,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConnectionGroupDataAttributesRequestVisitor)
    }
}
