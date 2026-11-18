// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The service-account key pair authentication method configured on the account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountServiceAccountAuthResponse {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
    /// ID of the public key half of the authorized key pair. Omitted when unset.
    #[serde(rename = "public_key_id")]
    pub public_key_id: Option<String>,
    /// ID of the Nebius service account the account authenticates as.
    #[serde(rename = "service_account_id")]
    pub service_account_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountServiceAccountAuthResponse {
    pub fn new(
        auth_type: crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
        service_account_id: String,
    ) -> NebiusCloudIntegrationAccountServiceAccountAuthResponse {
        NebiusCloudIntegrationAccountServiceAccountAuthResponse {
            auth_type,
            public_key_id: None,
            service_account_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn public_key_id(mut self, value: String) -> Self {
        self.public_key_id = Some(value);
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

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountServiceAccountAuthResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountServiceAccountAuthResponseVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountServiceAccountAuthResponseVisitor {
            type Value = NebiusCloudIntegrationAccountServiceAccountAuthResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
                > = None;
                let mut public_key_id: Option<String> = None;
                let mut service_account_id: Option<String> = None;
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
                                    crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "public_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account_id" => {
                            service_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let service_account_id = service_account_id
                    .ok_or_else(|| M::Error::missing_field("service_account_id"))?;

                let content = NebiusCloudIntegrationAccountServiceAccountAuthResponse {
                    auth_type,
                    public_key_id,
                    service_account_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountServiceAccountAuthResponseVisitor)
    }
}
