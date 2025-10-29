// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Basic information about a deployment gate.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGateResponseDataAttributes {
    /// The timestamp when the deployment gate was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Information about the user who created the deployment gate.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::DeploymentGateResponseDataAttributesCreatedBy,
    /// Whether this gate is run in dry-run mode.
    #[serde(rename = "dry_run")]
    pub dry_run: bool,
    /// The environment of the deployment gate.
    #[serde(rename = "env")]
    pub env: String,
    /// The identifier of the deployment gate.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// The service of the deployment gate.
    #[serde(rename = "service")]
    pub service: String,
    /// The timestamp when the deployment gate was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Information about the user who updated the deployment gate.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<crate::datadogV2::model::DeploymentGateResponseDataAttributesUpdatedBy>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGateResponseDataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: crate::datadogV2::model::DeploymentGateResponseDataAttributesCreatedBy,
        dry_run: bool,
        env: String,
        identifier: String,
        service: String,
    ) -> DeploymentGateResponseDataAttributes {
        DeploymentGateResponseDataAttributes {
            created_at,
            created_by,
            dry_run,
            env,
            identifier,
            service,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(
        mut self,
        value: crate::datadogV2::model::DeploymentGateResponseDataAttributesUpdatedBy,
    ) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for DeploymentGateResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGateResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for DeploymentGateResponseDataAttributesVisitor {
            type Value = DeploymentGateResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<
                    crate::datadogV2::model::DeploymentGateResponseDataAttributesCreatedBy,
                > = None;
                let mut dry_run: Option<bool> = None;
                let mut env: Option<String> = None;
                let mut identifier: Option<String> = None;
                let mut service: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<
                    crate::datadogV2::model::DeploymentGateResponseDataAttributesUpdatedBy,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dry_run" => {
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "identifier" => {
                            identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let dry_run = dry_run.ok_or_else(|| M::Error::missing_field("dry_run"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let identifier = identifier.ok_or_else(|| M::Error::missing_field("identifier"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;

                let content = DeploymentGateResponseDataAttributes {
                    created_at,
                    created_by,
                    dry_run,
                    env,
                    identifier,
                    service,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGateResponseDataAttributesVisitor)
    }
}
