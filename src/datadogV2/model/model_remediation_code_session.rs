// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A linked code session (for example, a pull request) for the investigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationCodeSession {
    /// Code session ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Pull request status for a linked code session.
    #[serde(rename = "pull_request_status")]
    pub pull_request_status: Option<crate::datadogV2::model::RemediationPullRequestStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationCodeSession {
    pub fn new() -> RemediationCodeSession {
        RemediationCodeSession {
            id: None,
            pull_request_status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn pull_request_status(
        mut self,
        value: crate::datadogV2::model::RemediationPullRequestStatus,
    ) -> Self {
        self.pull_request_status = Some(value);
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

impl Default for RemediationCodeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationCodeSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationCodeSessionVisitor;
        impl<'a> Visitor<'a> for RemediationCodeSessionVisitor {
            type Value = RemediationCodeSession;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut pull_request_status: Option<
                    crate::datadogV2::model::RemediationPullRequestStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pull_request_status" => {
                            if v.is_null() {
                                continue;
                            }
                            pull_request_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _pull_request_status) = pull_request_status {
                                match _pull_request_status {
                                    crate::datadogV2::model::RemediationPullRequestStatus::UnparsedObject(_pull_request_status) => {
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

                let content = RemediationCodeSession {
                    id,
                    pull_request_status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationCodeSessionVisitor)
    }
}
