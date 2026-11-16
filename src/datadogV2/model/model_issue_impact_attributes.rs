// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The issue's errors in the time window, and the users, sessions, accounts, views, and endpoints they affect.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactAttributes {
    /// Accounts whose users are affected by the issue.
    #[serde(rename = "accounts")]
    pub accounts: crate::datadogV2::model::IssueImpactAccounts,
    /// Endpoints whose traces have the issue's errors, identified by each trace's root span.
    #[serde(rename = "endpoints")]
    pub endpoints: crate::datadogV2::model::IssueImpactEndpoints,
    /// Number of errors reported by each source. Several sources can report the same error, such as an APM span and
    /// a log, so the counts overlap: don't add them together.
    #[serde(rename = "error_counts")]
    pub error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
    /// RUM sessions where the issue occurred.
    #[serde(rename = "sessions")]
    pub sessions: crate::datadogV2::model::IssueImpactSessions,
    /// Users affected by the issue.
    #[serde(rename = "users")]
    pub users: crate::datadogV2::model::IssueImpactUsers,
    /// RUM views where the issue occurred.
    #[serde(rename = "views")]
    pub views: crate::datadogV2::model::IssueImpactViews,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueImpactAttributes {
    pub fn new(
        accounts: crate::datadogV2::model::IssueImpactAccounts,
        endpoints: crate::datadogV2::model::IssueImpactEndpoints,
        error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
        sessions: crate::datadogV2::model::IssueImpactSessions,
        users: crate::datadogV2::model::IssueImpactUsers,
        views: crate::datadogV2::model::IssueImpactViews,
    ) -> IssueImpactAttributes {
        IssueImpactAttributes {
            accounts,
            endpoints,
            error_counts,
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

impl<'de> Deserialize<'de> for IssueImpactAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactAttributesVisitor;
        impl<'a> Visitor<'a> for IssueImpactAttributesVisitor {
            type Value = IssueImpactAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<crate::datadogV2::model::IssueImpactAccounts> = None;
                let mut endpoints: Option<crate::datadogV2::model::IssueImpactEndpoints> = None;
                let mut error_counts: Option<crate::datadogV2::model::IssueImpactErrorCounts> =
                    None;
                let mut sessions: Option<crate::datadogV2::model::IssueImpactSessions> = None;
                let mut users: Option<crate::datadogV2::model::IssueImpactUsers> = None;
                let mut views: Option<crate::datadogV2::model::IssueImpactViews> = None;
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
                        "endpoints" => {
                            endpoints = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_counts" => {
                            error_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let endpoints = endpoints.ok_or_else(|| M::Error::missing_field("endpoints"))?;
                let error_counts =
                    error_counts.ok_or_else(|| M::Error::missing_field("error_counts"))?;
                let sessions = sessions.ok_or_else(|| M::Error::missing_field("sessions"))?;
                let users = users.ok_or_else(|| M::Error::missing_field("users"))?;
                let views = views.ok_or_else(|| M::Error::missing_field("views"))?;

                let content = IssueImpactAttributes {
                    accounts,
                    endpoints,
                    error_counts,
                    sessions,
                    users,
                    views,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactAttributesVisitor)
    }
}
