// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the information of a pull request linked to an issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssuePullRequestAttributes {
    /// Timestamp of when the pull request was closed. Omitted if the pull request is not closed.
    #[serde(rename = "closed_at")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the pull request was linked to the issue.
    #[serde(rename = "linked_at")]
    pub linked_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the pull request was merged. Omitted if the pull request is not merged.
    #[serde(rename = "merged_at")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Why the pull request is linked to the issue. `fix` means the pull request fixes the issue, and `trigger` means the pull request introduced the issue.
    #[serde(rename = "reason")]
    pub reason: Option<crate::datadogV2::model::IssuePullRequestReason>,
    /// URL of the pull request.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssuePullRequestAttributes {
    pub fn new() -> IssuePullRequestAttributes {
        IssuePullRequestAttributes {
            closed_at: None,
            linked_at: None,
            merged_at: None,
            reason: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn closed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.closed_at = Some(value);
        self
    }

    pub fn linked_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.linked_at = Some(value);
        self
    }

    pub fn merged_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.merged_at = Some(value);
        self
    }

    pub fn reason(mut self, value: crate::datadogV2::model::IssuePullRequestReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for IssuePullRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssuePullRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssuePullRequestAttributesVisitor;
        impl<'a> Visitor<'a> for IssuePullRequestAttributesVisitor {
            type Value = IssuePullRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut closed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut linked_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut merged_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut reason: Option<crate::datadogV2::model::IssuePullRequestReason> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "closed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            closed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "linked_at" => {
                            if v.is_null() {
                                continue;
                            }
                            linked_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "merged_at" => {
                            if v.is_null() {
                                continue;
                            }
                            merged_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reason) = reason {
                                match _reason {
                                    crate::datadogV2::model::IssuePullRequestReason::UnparsedObject(_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssuePullRequestAttributes {
                    closed_at,
                    linked_at,
                    merged_at,
                    reason,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssuePullRequestAttributesVisitor)
    }
}
