// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fleet access scope attributes and audit information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAccessScopeResponseAttributes {
    /// RFC3339 timestamp when the scope was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Handle of the user who created the scope.
    #[serde(rename = "creator")]
    pub creator: Option<String>,
    /// Human-readable description of the access scope. Omission clears the description on update.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Fleet actions authorized on agents matching the access scope query.
    #[serde(rename = "features")]
    pub features: Option<crate::datadogV2::model::FleetAccessScopeFeatures>,
    /// Agent query selecting hosts covered by the scope. An empty query matches all hosts. Omission clears the query on update.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// RFC3339 timestamp when the scope was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Handle of the user who last updated the scope.
    #[serde(rename = "updater")]
    pub updater: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAccessScopeResponseAttributes {
    pub fn new() -> FleetAccessScopeResponseAttributes {
        FleetAccessScopeResponseAttributes {
            created_at: None,
            creator: None,
            description: None,
            features: None,
            query: None,
            updated_at: None,
            updater: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator(mut self, value: String) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn features(mut self, value: crate::datadogV2::model::FleetAccessScopeFeatures) -> Self {
        self.features = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updater(mut self, value: String) -> Self {
        self.updater = Some(value);
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

impl Default for FleetAccessScopeResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAccessScopeResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAccessScopeResponseAttributesVisitor;
        impl<'a> Visitor<'a> for FleetAccessScopeResponseAttributesVisitor {
            type Value = FleetAccessScopeResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut creator: Option<String> = None;
                let mut description: Option<String> = None;
                let mut features: Option<crate::datadogV2::model::FleetAccessScopeFeatures> = None;
                let mut query: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut updater: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "features" => {
                            if v.is_null() {
                                continue;
                            }
                            features = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updater" => {
                            if v.is_null() {
                                continue;
                            }
                            updater = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetAccessScopeResponseAttributes {
                    created_at,
                    creator,
                    description,
                    features,
                    query,
                    updated_at,
                    updater,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAccessScopeResponseAttributesVisitor)
    }
}
