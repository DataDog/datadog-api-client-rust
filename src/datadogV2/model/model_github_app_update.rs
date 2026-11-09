// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update to GitHub App credentials.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GithubAppUpdate {
    /// The numeric ID of the GitHub App.
    #[serde(rename = "github_app_id")]
    pub github_app_id: Option<crate::datadogV2::model::GithubAppUpdateGithubAppId>,
    /// The installation ID of the GitHub App.
    #[serde(rename = "github_installation_id")]
    pub github_installation_id:
        Option<crate::datadogV2::model::GithubAppUpdateGithubInstallationId>,
    /// Hostname of the GitHub Enterprise Server. Omit this field for github.com.
    #[serde(rename = "hostname")]
    pub hostname: Option<crate::datadogV2::model::GithubAppUpdateHostname>,
    /// PEM-encoded RSA private key for the GitHub App.
    #[serde(rename = "private_key")]
    pub private_key: Option<crate::datadogV2::model::GithubAppUpdatePrivateKey>,
    /// The credential type for `GithubApp`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::GithubAppType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GithubAppUpdate {
    pub fn new(type_: crate::datadogV2::model::GithubAppType) -> GithubAppUpdate {
        GithubAppUpdate {
            github_app_id: None,
            github_installation_id: None,
            hostname: None,
            private_key: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn github_app_id(
        mut self,
        value: crate::datadogV2::model::GithubAppUpdateGithubAppId,
    ) -> Self {
        self.github_app_id = Some(value);
        self
    }

    pub fn github_installation_id(
        mut self,
        value: crate::datadogV2::model::GithubAppUpdateGithubInstallationId,
    ) -> Self {
        self.github_installation_id = Some(value);
        self
    }

    pub fn hostname(mut self, value: crate::datadogV2::model::GithubAppUpdateHostname) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn private_key(
        mut self,
        value: crate::datadogV2::model::GithubAppUpdatePrivateKey,
    ) -> Self {
        self.private_key = Some(value);
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

impl<'de> Deserialize<'de> for GithubAppUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GithubAppUpdateVisitor;
        impl<'a> Visitor<'a> for GithubAppUpdateVisitor {
            type Value = GithubAppUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut github_app_id: Option<crate::datadogV2::model::GithubAppUpdateGithubAppId> =
                    None;
                let mut github_installation_id: Option<
                    crate::datadogV2::model::GithubAppUpdateGithubInstallationId,
                > = None;
                let mut hostname: Option<crate::datadogV2::model::GithubAppUpdateHostname> = None;
                let mut private_key: Option<crate::datadogV2::model::GithubAppUpdatePrivateKey> =
                    None;
                let mut type_: Option<crate::datadogV2::model::GithubAppType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "github_app_id" => {
                            if v.is_null() {
                                continue;
                            }
                            github_app_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _github_app_id) = github_app_id {
                                match _github_app_id {
                                    crate::datadogV2::model::GithubAppUpdateGithubAppId::UnparsedObject(_github_app_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "github_installation_id" => {
                            if v.is_null() {
                                continue;
                            }
                            github_installation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _github_installation_id) = github_installation_id {
                                match _github_installation_id {
                                    crate::datadogV2::model::GithubAppUpdateGithubInstallationId::UnparsedObject(_github_installation_id) => {
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
                                    crate::datadogV2::model::GithubAppUpdateHostname::UnparsedObject(_hostname) => {
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
                            if let Some(ref _private_key) = private_key {
                                match _private_key {
                                    crate::datadogV2::model::GithubAppUpdatePrivateKey::UnparsedObject(_private_key) => {
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
                                    crate::datadogV2::model::GithubAppType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = GithubAppUpdate {
                    github_app_id,
                    github_installation_id,
                    hostname,
                    private_key,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GithubAppUpdateVisitor)
    }
}
