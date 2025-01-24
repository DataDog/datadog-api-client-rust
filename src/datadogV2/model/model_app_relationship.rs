// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The app's publication relationship and custom connections.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AppRelationship {
    /// Array of custom connections used by the app.
    #[serde(rename = "connections")]
    pub connections: Option<Vec<crate::datadogV2::model::CustomConnection>>,
    /// Information pointing to the app's publication status.
    #[serde(rename = "deployment")]
    pub deployment: Option<crate::datadogV2::model::DeploymentRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AppRelationship {
    pub fn new() -> AppRelationship {
        AppRelationship {
            connections: None,
            deployment: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn connections(mut self, value: Vec<crate::datadogV2::model::CustomConnection>) -> Self {
        self.connections = Some(value);
        self
    }

    pub fn deployment(mut self, value: crate::datadogV2::model::DeploymentRelationship) -> Self {
        self.deployment = Some(value);
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

impl Default for AppRelationship {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AppRelationship {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AppRelationshipVisitor;
        impl<'a> Visitor<'a> for AppRelationshipVisitor {
            type Value = AppRelationship;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connections: Option<Vec<crate::datadogV2::model::CustomConnection>> = None;
                let mut deployment: Option<crate::datadogV2::model::DeploymentRelationship> = None;
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
                        "deployment" => {
                            if v.is_null() {
                                continue;
                            }
                            deployment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AppRelationship {
                    connections,
                    deployment,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AppRelationshipVisitor)
    }
}
