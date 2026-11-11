// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service-account key pair authentication, the only method Nebius Cloud integration accounts support. Create a service account in the Nebius console, add it to the admin group, upload an authorized key pair to it, and issue a static API key with the OBSERVABILITY scope through the Nebius CLI.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountServiceAccountAuthRequest {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
    /// The private key of the authorized key pair, in PEM format. Required when log, resource, or service-quota collection is enabled; a request that enables one of them without it is rejected with a `422` response.
    #[serde(rename = "private_key")]
    pub private_key: Option<String>,
    /// ID of the public key half of the authorized key pair, such as `publickey-xxxxxxxx`. Required when log, resource, or service-quota collection is enabled; a request that enables one of them without it is rejected with a `422` response.
    #[serde(rename = "public_key_id")]
    pub public_key_id: Option<String>,
    /// Non-secret ID of the Nebius service account the account authenticates as.
    #[serde(rename = "service_account_id")]
    pub service_account_id: String,
    /// Nebius static API key with the OBSERVABILITY scope, used to query logs. Required when log, resource, or service-quota collection is enabled; a request that enables one of them without it is rejected with a `422` response.
    #[serde(rename = "static_api_key")]
    pub static_api_key: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountServiceAccountAuthRequest {
    pub fn new(
        auth_type: crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
        service_account_id: String,
    ) -> NebiusCloudIntegrationAccountServiceAccountAuthRequest {
        NebiusCloudIntegrationAccountServiceAccountAuthRequest {
            auth_type,
            private_key: None,
            public_key_id: None,
            service_account_id,
            static_api_key: None,
            _unparsed: false,
        }
    }

    pub fn private_key(mut self, value: String) -> Self {
        self.private_key = Some(value);
        self
    }

    pub fn public_key_id(mut self, value: String) -> Self {
        self.public_key_id = Some(value);
        self
    }

    pub fn static_api_key(mut self, value: String) -> Self {
        self.static_api_key = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountServiceAccountAuthRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountServiceAccountAuthRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountServiceAccountAuthRequestVisitor {
            type Value = NebiusCloudIntegrationAccountServiceAccountAuthRequest;

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
                let mut private_key: Option<String> = None;
                let mut public_key_id: Option<String> = None;
                let mut service_account_id: Option<String> = None;
                let mut static_api_key: Option<String> = None;
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
                        "private_key" => {
                            if v.is_null() {
                                continue;
                            }
                            private_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "static_api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            static_api_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let service_account_id = service_account_id
                    .ok_or_else(|| M::Error::missing_field("service_account_id"))?;

                let content = NebiusCloudIntegrationAccountServiceAccountAuthRequest {
                    auth_type,
                    private_key,
                    public_key_id,
                    service_account_id,
                    static_api_key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountServiceAccountAuthRequestVisitor)
    }
}
