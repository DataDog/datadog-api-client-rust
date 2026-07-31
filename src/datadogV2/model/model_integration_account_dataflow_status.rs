// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Read-only, server-computed collection status of a dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationAccountDataflowStatus {
    /// Collection health of a single dataflow.
    #[serde(rename = "health")]
    pub health: Option<crate::datadogV2::model::IntegrationAccountDataflowHealth>,
    /// Human-readable detail, populated when the dataflow is not healthy.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Time the status was last computed.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAccountDataflowStatus {
    pub fn new() -> IntegrationAccountDataflowStatus {
        IntegrationAccountDataflowStatus {
            health: None,
            message: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn health(
        mut self,
        value: crate::datadogV2::model::IntegrationAccountDataflowHealth,
    ) -> Self {
        self.health = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
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

impl Default for IntegrationAccountDataflowStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationAccountDataflowStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAccountDataflowStatusVisitor;
        impl<'a> Visitor<'a> for IntegrationAccountDataflowStatusVisitor {
            type Value = IntegrationAccountDataflowStatus;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut health: Option<crate::datadogV2::model::IntegrationAccountDataflowHealth> =
                    None;
                let mut message: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "health" => {
                            if v.is_null() {
                                continue;
                            }
                            health = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _health) = health {
                                match _health {
                                    crate::datadogV2::model::IntegrationAccountDataflowHealth::UnparsedObject(_health) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationAccountDataflowStatus {
                    health,
                    message,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAccountDataflowStatusVisitor)
    }
}
