// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing a GitHub account's CI Visibility opt-in status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppGitHubAccountAttributes {
    /// The GitHub account (organization or user) name.
    #[serde(rename = "account")]
    pub account: Option<String>,
    /// Whether CI Visibility is enabled at the account level.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The GitHub host (`github.com` or a GitHub Enterprise Server (GHES) hostname) this account belongs to.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The number of repositories known for this account.
    #[serde(rename = "repo_count")]
    pub repo_count: Option<i64>,
    /// The repositories belonging to this account, with their individual opt-in status.
    #[serde(rename = "repositories")]
    pub repositories: Option<Vec<crate::datadogV2::model::CIAppGitHubAccountRepository>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppGitHubAccountAttributes {
    pub fn new() -> CIAppGitHubAccountAttributes {
        CIAppGitHubAccountAttributes {
            account: None,
            enabled: None,
            host: None,
            repo_count: None,
            repositories: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account(mut self, value: String) -> Self {
        self.account = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn repo_count(mut self, value: i64) -> Self {
        self.repo_count = Some(value);
        self
    }

    pub fn repositories(
        mut self,
        value: Vec<crate::datadogV2::model::CIAppGitHubAccountRepository>,
    ) -> Self {
        self.repositories = Some(value);
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

impl Default for CIAppGitHubAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppGitHubAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppGitHubAccountAttributesVisitor;
        impl<'a> Visitor<'a> for CIAppGitHubAccountAttributesVisitor {
            type Value = CIAppGitHubAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut host: Option<String> = None;
                let mut repo_count: Option<i64> = None;
                let mut repositories: Option<
                    Vec<crate::datadogV2::model::CIAppGitHubAccountRepository>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account" => {
                            if v.is_null() {
                                continue;
                            }
                            account = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repo_count" => {
                            if v.is_null() {
                                continue;
                            }
                            repo_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repositories" => {
                            if v.is_null() {
                                continue;
                            }
                            repositories =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CIAppGitHubAccountAttributes {
                    account,
                    enabled,
                    host,
                    repo_count,
                    repositories,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppGitHubAccountAttributesVisitor)
    }
}
