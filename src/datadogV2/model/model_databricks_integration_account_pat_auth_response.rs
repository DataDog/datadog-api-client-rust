// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Databricks personal access token authentication method configured on the account. Deprecated: migrate these accounts to `databricks-oauth` or `private-action-runner`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationAccountPatAuthResponse {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationAccountPatAuthResponse {
    pub fn new(
        auth_type: crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType,
    ) -> DatabricksIntegrationAccountPatAuthResponse {
        DatabricksIntegrationAccountPatAuthResponse {
            auth_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountPatAuthResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationAccountPatAuthResponseVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationAccountPatAuthResponseVisitor {
            type Value = DatabricksIntegrationAccountPatAuthResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;

                let content = DatabricksIntegrationAccountPatAuthResponse {
                    auth_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationAccountPatAuthResponseVisitor)
    }
}
