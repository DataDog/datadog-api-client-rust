// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request to exchange a Vercel marketplace authorization code for a Datadog-managed access token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VercelTokenCreateRequest {
    /// OAuth authorization code received from the Vercel marketplace flow.
    #[serde(rename = "authGrantCode")]
    pub auth_grant_code: String,
    /// Vercel configuration identifier returned by the marketplace flow.
    #[serde(rename = "vercelConfigurationId")]
    pub vercel_configuration_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl VercelTokenCreateRequest {
    pub fn new(
        auth_grant_code: String,
        vercel_configuration_id: String,
    ) -> VercelTokenCreateRequest {
        VercelTokenCreateRequest {
            auth_grant_code,
            vercel_configuration_id,
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

impl<'de> Deserialize<'de> for VercelTokenCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VercelTokenCreateRequestVisitor;
        impl<'a> Visitor<'a> for VercelTokenCreateRequestVisitor {
            type Value = VercelTokenCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_grant_code: Option<String> = None;
                let mut vercel_configuration_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authGrantCode" => {
                            auth_grant_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vercelConfigurationId" => {
                            vercel_configuration_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_grant_code =
                    auth_grant_code.ok_or_else(|| M::Error::missing_field("auth_grant_code"))?;
                let vercel_configuration_id = vercel_configuration_id
                    .ok_or_else(|| M::Error::missing_field("vercel_configuration_id"))?;

                let content = VercelTokenCreateRequest {
                    auth_grant_code,
                    vercel_configuration_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(VercelTokenCreateRequestVisitor)
    }
}
