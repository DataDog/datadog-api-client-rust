// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Contains the identifiers and URL for a successfully created Linear issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueCaseLinearIssueResult {
    /// Linear account identifier.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Linear issue identifier.
    #[serde(rename = "issue_id")]
    pub issue_id: Option<String>,
    /// Linear issue key.
    #[serde(rename = "issue_key")]
    pub issue_key: Option<String>,
    /// Linear issue URL.
    #[serde(rename = "issue_url")]
    pub issue_url: Option<String>,
    /// Linear team identifier.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueCaseLinearIssueResult {
    pub fn new() -> IssueCaseLinearIssueResult {
        IssueCaseLinearIssueResult {
            account_id: None,
            issue_id: None,
            issue_key: None,
            issue_url: None,
            team_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn issue_id(mut self, value: String) -> Self {
        self.issue_id = Some(value);
        self
    }

    pub fn issue_key(mut self, value: String) -> Self {
        self.issue_key = Some(value);
        self
    }

    pub fn issue_url(mut self, value: String) -> Self {
        self.issue_url = Some(value);
        self
    }

    pub fn team_id(mut self, value: String) -> Self {
        self.team_id = Some(value);
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

impl Default for IssueCaseLinearIssueResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueCaseLinearIssueResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueCaseLinearIssueResultVisitor;
        impl<'a> Visitor<'a> for IssueCaseLinearIssueResultVisitor {
            type Value = IssueCaseLinearIssueResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut issue_id: Option<String> = None;
                let mut issue_key: Option<String> = None;
                let mut issue_url: Option<String> = None;
                let mut team_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_id" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_key" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_url" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssueCaseLinearIssueResult {
                    account_id,
                    issue_id,
                    issue_key,
                    issue_url,
                    team_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueCaseLinearIssueResultVisitor)
    }
}
