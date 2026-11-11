// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service-account key pair authentication, the only method Nebius Cloud integration accounts support. Only the fields provided are changed; omit `private_key` and `static_api_key` to keep the stored ones.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountServiceAccountAuthUpdate {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
    /// The private key of the authorized key pair, in PEM format. Omit it to keep the stored one; an empty string is rejected with a `422` response.
    #[serde(rename = "private_key")]
    pub private_key: Option<String>,
    /// ID of the public key half of the authorized key pair. Omit it to keep the stored public key ID, send `null` or an empty string to remove it, or send a value to replace it. Removing it is rejected with a `422` response while log, resource, or service-quota collection is enabled.
    #[serde(
        rename = "public_key_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub public_key_id: Option<Option<String>>,
    /// Non-secret ID of the Nebius service account the account authenticates as. It cannot be cleared; sending `null` or an empty string is rejected with a `422` response.
    #[serde(rename = "service_account_id")]
    pub service_account_id: Option<String>,
    /// Nebius static API key with the OBSERVABILITY scope. Omit it to keep the stored one; an empty string is rejected with a `422` response.
    #[serde(rename = "static_api_key")]
    pub static_api_key: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountServiceAccountAuthUpdate {
    pub fn new(
        auth_type: crate::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType,
    ) -> NebiusCloudIntegrationAccountServiceAccountAuthUpdate {
        NebiusCloudIntegrationAccountServiceAccountAuthUpdate {
            auth_type,
            private_key: None,
            public_key_id: None,
            service_account_id: None,
            static_api_key: None,
            _unparsed: false,
        }
    }

    pub fn private_key(mut self, value: String) -> Self {
        self.private_key = Some(value);
        self
    }

    pub fn public_key_id(mut self, value: Option<String>) -> Self {
        self.public_key_id = Some(value);
        self
    }

    pub fn service_account_id(mut self, value: String) -> Self {
        self.service_account_id = Some(value);
        self
    }

    pub fn static_api_key(mut self, value: String) -> Self {
        self.static_api_key = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountServiceAccountAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountServiceAccountAuthUpdateVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountServiceAccountAuthUpdateVisitor {
            type Value = NebiusCloudIntegrationAccountServiceAccountAuthUpdate;

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
                let mut public_key_id: Option<Option<String>> = None;
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
                            public_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account_id" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = NebiusCloudIntegrationAccountServiceAccountAuthUpdate {
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

        deserializer.deserialize_any(NebiusCloudIntegrationAccountServiceAccountAuthUpdateVisitor)
    }
}
