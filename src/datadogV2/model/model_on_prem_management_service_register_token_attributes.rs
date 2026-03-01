// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for registering a token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnPremManagementServiceRegisterTokenAttributes {
    /// The application identifier.
    #[serde(rename = "app_id")]
    pub app_id: Option<uuid::Uuid>,
    /// The application version.
    #[serde(rename = "app_version")]
    pub app_version: Option<i64>,
    /// The connection identifier.
    #[serde(rename = "connection_id")]
    pub connection_id: uuid::Uuid,
    /// The query identifier.
    #[serde(rename = "query_id")]
    pub query_id: Option<uuid::Uuid>,
    /// The on-prem runner identifier.
    #[serde(rename = "runner_id")]
    pub runner_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnPremManagementServiceRegisterTokenAttributes {
    pub fn new(
        connection_id: uuid::Uuid,
        runner_id: String,
    ) -> OnPremManagementServiceRegisterTokenAttributes {
        OnPremManagementServiceRegisterTokenAttributes {
            app_id: None,
            app_version: None,
            connection_id,
            query_id: None,
            runner_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn app_id(mut self, value: uuid::Uuid) -> Self {
        self.app_id = Some(value);
        self
    }

    pub fn app_version(mut self, value: i64) -> Self {
        self.app_version = Some(value);
        self
    }

    pub fn query_id(mut self, value: uuid::Uuid) -> Self {
        self.query_id = Some(value);
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

impl<'de> Deserialize<'de> for OnPremManagementServiceRegisterTokenAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnPremManagementServiceRegisterTokenAttributesVisitor;
        impl<'a> Visitor<'a> for OnPremManagementServiceRegisterTokenAttributesVisitor {
            type Value = OnPremManagementServiceRegisterTokenAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut app_id: Option<uuid::Uuid> = None;
                let mut app_version: Option<i64> = None;
                let mut connection_id: Option<uuid::Uuid> = None;
                let mut query_id: Option<uuid::Uuid> = None;
                let mut runner_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "app_id" => {
                            if v.is_null() {
                                continue;
                            }
                            app_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "app_version" => {
                            if v.is_null() {
                                continue;
                            }
                            app_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connection_id" => {
                            connection_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_id" => {
                            if v.is_null() {
                                continue;
                            }
                            query_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_id" => {
                            runner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let connection_id =
                    connection_id.ok_or_else(|| M::Error::missing_field("connection_id"))?;
                let runner_id = runner_id.ok_or_else(|| M::Error::missing_field("runner_id"))?;

                let content = OnPremManagementServiceRegisterTokenAttributes {
                    app_id,
                    app_version,
                    connection_id,
                    query_id,
                    runner_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnPremManagementServiceRegisterTokenAttributesVisitor)
    }
}
