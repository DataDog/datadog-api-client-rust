// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a GitHub account's CI Visibility opt-in status.
/// At least one of `enabled` or `repository.enabled` must be provided.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppGitHubAccountUpdateRequestAttributes {
    /// The GitHub account (organization or user) name to update, identified by name.
    #[serde(rename = "account")]
    pub account: String,
    /// Whether to enable or disable CI Visibility at the account level.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The GitHub host (`github.com` or a GHES hostname) the account belongs to. Required to disambiguate
    /// when the same account name exists on more than one host.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// Repository-level opt-in change to apply, identified by name.
    #[serde(rename = "repository")]
    pub repository: Option<crate::datadogV2::model::CIAppGitHubAccountUpdateRequestRepository>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppGitHubAccountUpdateRequestAttributes {
    pub fn new(account: String) -> CIAppGitHubAccountUpdateRequestAttributes {
        CIAppGitHubAccountUpdateRequestAttributes {
            account,
            enabled: None,
            host: None,
            repository: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn repository(
        mut self,
        value: crate::datadogV2::model::CIAppGitHubAccountUpdateRequestRepository,
    ) -> Self {
        self.repository = Some(value);
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

impl<'de> Deserialize<'de> for CIAppGitHubAccountUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppGitHubAccountUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CIAppGitHubAccountUpdateRequestAttributesVisitor {
            type Value = CIAppGitHubAccountUpdateRequestAttributes;

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
                let mut repository: Option<
                    crate::datadogV2::model::CIAppGitHubAccountUpdateRequestRepository,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account" => {
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
                        "repository" => {
                            if v.is_null() {
                                continue;
                            }
                            repository = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account = account.ok_or_else(|| M::Error::missing_field("account"))?;

                let content = CIAppGitHubAccountUpdateRequestAttributes {
                    account,
                    enabled,
                    host,
                    repository,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppGitHubAccountUpdateRequestAttributesVisitor)
    }
}
