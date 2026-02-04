// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a connection group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConnectionGroupDataAttributesResponse {
    /// List of connection IDs associated with the connection group.
    #[serde(rename = "connections")]
    pub connections: Option<Vec<String>>,
    /// The creation timestamp of the connection group.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The description of the connection group.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The integration type of the connection group.
    #[serde(rename = "integration_type")]
    pub integration_type: String,
    /// Indicates if the connection group is marked as favorite.
    #[serde(rename = "is_favorite")]
    pub is_favorite: bool,
    /// The last updated timestamp of the connection group.
    #[serde(rename = "last_updated_at")]
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    /// The name of the connection group.
    #[serde(rename = "name")]
    pub name: String,
    /// Tag keys associated with the connection group.
    #[serde(rename = "tag_keys")]
    pub tag_keys: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConnectionGroupDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        integration_type: String,
        is_favorite: bool,
        last_updated_at: chrono::DateTime<chrono::Utc>,
        name: String,
        tag_keys: Vec<String>,
    ) -> ConnectionGroupDataAttributesResponse {
        ConnectionGroupDataAttributesResponse {
            connections: None,
            created_at,
            description: None,
            integration_type,
            is_favorite,
            last_updated_at,
            name,
            tag_keys,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ConnectionGroupDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConnectionGroupDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for ConnectionGroupDataAttributesResponseVisitor {
            type Value = ConnectionGroupDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connections: Option<Vec<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut integration_type: Option<String> = None;
                let mut is_favorite: Option<bool> = None;
                let mut last_updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorite" => {
                            is_favorite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_at" => {
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_keys" => {
                            tag_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let integration_type =
                    integration_type.ok_or_else(|| M::Error::missing_field("integration_type"))?;
                let is_favorite =
                    is_favorite.ok_or_else(|| M::Error::missing_field("is_favorite"))?;
                let last_updated_at =
                    last_updated_at.ok_or_else(|| M::Error::missing_field("last_updated_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tag_keys = tag_keys.ok_or_else(|| M::Error::missing_field("tag_keys"))?;

                let content = ConnectionGroupDataAttributesResponse {
                    connections,
                    created_at,
                    description,
                    integration_type,
                    is_favorite,
                    last_updated_at,
                    name,
                    tag_keys,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConnectionGroupDataAttributesResponseVisitor)
    }
}
