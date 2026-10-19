// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Bearer token authentication. Use the API key of a Temporal Cloud service account with the Metrics Read-Only account-level role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type:
        crate::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType,
    /// Secret token used to authenticate with Temporal Cloud.
    #[serde(rename = "token")]
    pub token: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest {
    pub fn new(
        auth_type: crate::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType,
        token: String,
    ) -> TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest {
        TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest {
            auth_type,
            token,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequestVisitor;
        impl<'a> Visitor<'a> for TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequestVisitor {
            type Value = TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<crate::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType> = None;
                let mut token: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "token" => {
                            token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let token = token.ok_or_else(|| M::Error::missing_field("token"))?;

                let content = TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest {
                    auth_type,
                    token,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequestVisitor,
        )
    }
}
