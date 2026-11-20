// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// API token authentication, the only method CoreWeave integration accounts support. Only the fields provided are changed; omit `api_token` to keep the stored one.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate {
    /// Secret CoreWeave API access token used to authenticate with CoreWeave. It cannot be cleared; sending `null` or an empty string is rejected with a `422` response.
    #[serde(rename = "api_token")]
    pub api_token: Option<String>,
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate {
    pub fn new(auth_type: crate::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType) -> CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate {
        CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate {
            api_token: None,
            auth_type,
            _unparsed: false,
        }
    }
    
    pub fn api_token(mut self, value: String) -> Self {
        self.api_token = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudIntegrationAccountAPITokenAuthUpdateVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudIntegrationAccountAPITokenAuthUpdateVisitor {
            type Value = CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate;

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
                            if v.is_null() {
                                continue;
                            }
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
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;

                let content = CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate {
                    api_token,
                    auth_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudIntegrationAccountAPITokenAuthUpdateVisitor)
    }
}
