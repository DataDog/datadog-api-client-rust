// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Distinct users, accounts, sessions, and view instances on a view. `affected` counts only those with the issue's
/// errors. `baseline` counts all users, accounts, sessions, and view instances on the view in the time window. It
/// is absent when RUM has no data for the view.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactViewCounts {
    /// Number of distinct accounts.
    #[serde(rename = "accounts")]
    pub accounts: i64,
    /// Number of distinct sessions.
    #[serde(rename = "sessions")]
    pub sessions: i64,
    /// Number of distinct users.
    #[serde(rename = "users")]
    pub users: i64,
    /// Number of distinct view instances.
    #[serde(rename = "views")]
    pub views: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueImpactViewCounts {
    pub fn new(accounts: i64, sessions: i64, users: i64, views: i64) -> IssueImpactViewCounts {
        IssueImpactViewCounts {
            accounts,
            sessions,
            users,
            views,
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

impl<'de> Deserialize<'de> for IssueImpactViewCounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactViewCountsVisitor;
        impl<'a> Visitor<'a> for IssueImpactViewCountsVisitor {
            type Value = IssueImpactViewCounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<i64> = None;
                let mut sessions: Option<i64> = None;
                let mut users: Option<i64> = None;
                let mut views: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions" => {
                            sessions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "users" => {
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "views" => {
                            views = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let accounts = accounts.ok_or_else(|| M::Error::missing_field("accounts"))?;
                let sessions = sessions.ok_or_else(|| M::Error::missing_field("sessions"))?;
                let users = users.ok_or_else(|| M::Error::missing_field("users"))?;
                let views = views.ok_or_else(|| M::Error::missing_field("views"))?;

                let content = IssueImpactViewCounts {
                    accounts,
                    sessions,
                    users,
                    views,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactViewCountsVisitor)
    }
}
