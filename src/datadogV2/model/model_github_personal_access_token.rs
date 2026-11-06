// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// GitHub personal access token credentials.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GithubPersonalAccessToken {
    /// GitHub personal access token.
    #[serde(rename = "access_token")]
    pub access_token: crate::datadogV2::model::GithubPersonalAccessTokenAccessToken,
    /// Hostname of the GitHub Enterprise Server. Omit this field for github.com.
    #[serde(rename = "hostname")]
    pub hostname: Option<crate::datadogV2::model::GithubPersonalAccessTokenHostname>,
    /// The credential type for `GithubPersonalAccessToken`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::GithubPersonalAccessTokenType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GithubPersonalAccessToken {
    pub fn new(
        access_token: crate::datadogV2::model::GithubPersonalAccessTokenAccessToken,
        type_: crate::datadogV2::model::GithubPersonalAccessTokenType,
    ) -> GithubPersonalAccessToken {
        GithubPersonalAccessToken {
            access_token,
            hostname: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hostname(
        mut self,
        value: crate::datadogV2::model::GithubPersonalAccessTokenHostname,
    ) -> Self {
        self.hostname = Some(value);
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

impl<'de> Deserialize<'de> for GithubPersonalAccessToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GithubPersonalAccessTokenVisitor;
        impl<'a> Visitor<'a> for GithubPersonalAccessTokenVisitor {
            type Value = GithubPersonalAccessToken;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_token: Option<
                    crate::datadogV2::model::GithubPersonalAccessTokenAccessToken,
                > = None;
                let mut hostname: Option<
                    crate::datadogV2::model::GithubPersonalAccessTokenHostname,
                > = None;
                let mut type_: Option<crate::datadogV2::model::GithubPersonalAccessTokenType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_token" => {
                            access_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _access_token) = access_token {
                                match _access_token {
                                    crate::datadogV2::model::GithubPersonalAccessTokenAccessToken::UnparsedObject(_access_token) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _hostname) = hostname {
                                match _hostname {
                                    crate::datadogV2::model::GithubPersonalAccessTokenHostname::UnparsedObject(_hostname) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::GithubPersonalAccessTokenType::UnparsedObject(_type_) => {
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
                let access_token =
                    access_token.ok_or_else(|| M::Error::missing_field("access_token"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = GithubPersonalAccessToken {
                    access_token,
                    hostname,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GithubPersonalAccessTokenVisitor)
    }
}
