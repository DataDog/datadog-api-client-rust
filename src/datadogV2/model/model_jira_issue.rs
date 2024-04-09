// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Jira issue attached to case
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraIssue {
    /// Jira issue information
    #[serde(rename = "result")]
    pub result: Option<crate::datadogV2::model::JiraIssueResult>,
    /// Case status
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::Case3rdPartyTicketStatus>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssue {
    pub fn new() -> JiraIssue {
        JiraIssue {
            result: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn result(mut self, value: crate::datadogV2::model::JiraIssueResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::Case3rdPartyTicketStatus) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for JiraIssue {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for JiraIssue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueVisitor;
        impl<'a> Visitor<'a> for JiraIssueVisitor {
            type Value = JiraIssue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut result: Option<crate::datadogV2::model::JiraIssueResult> = None;
                let mut status: Option<crate::datadogV2::model::Case3rdPartyTicketStatus> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "result" => {
                            if v.is_null() {
                                continue;
                            }
                            result = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::Case3rdPartyTicketStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = JiraIssue {
                    result,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueVisitor)
    }
}
