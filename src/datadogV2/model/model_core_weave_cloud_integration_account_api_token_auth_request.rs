// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// API token authentication, the only method CoreWeave integration accounts support. Create the token in the CoreWeave Cloud console under Access Tokens; it starts with `CW-SECRET-`. The user who owns the token must have the Observability Viewer role, because tokens inherit their user's permissions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudIntegrationAccountAPITokenAuthRequest {
    /// Secret CoreWeave API access token used to authenticate with CoreWeave. Values that do not start with `CW-SECRET-` are rejected with a `422` response.
    #[serde(rename = "api_token")]
    pub api_token: String,
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudIntegrationAccountAPITokenAuthRequest {
    pub fn new(api_token: String, auth_type: crate::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType) -> CoreWeaveCloudIntegrationAccountAPITokenAuthRequest {
        CoreWeaveCloudIntegrationAccountAPITokenAuthRequest {
            api_token,
            auth_type,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for CoreWeaveCloudIntegrationAccountAPITokenAuthRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudIntegrationAccountAPITokenAuthRequestVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudIntegrationAccountAPITokenAuthRequestVisitor {
            type Value = CoreWeaveCloudIntegrationAccountAPITokenAuthRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_token: Option<String> = None;
                let mut auth_type: Option<crate::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_token" => {
                            api_token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let api_token = api_token.ok_or_else(|| M::Error::missing_field("api_token"))?;
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;

                let content = CoreWeaveCloudIntegrationAccountAPITokenAuthRequest {
                    api_token,
                    auth_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudIntegrationAccountAPITokenAuthRequestVisitor)
    }
}
