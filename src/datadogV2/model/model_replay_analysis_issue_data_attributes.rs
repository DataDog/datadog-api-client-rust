// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a RUM replay analysis issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisIssueDataAttributes {
    /// Up to three sample sessions affected by this issue.
    #[serde(rename = "affected_sessions")]
    pub affected_sessions: Vec<crate::datadogV2::model::ReplayAnalysisAffectedSession>,
    /// Unique identifier of the application where the issue was detected.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Timestamp when the issue was first detected.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Human-readable description of the issue.
    #[serde(rename = "description")]
    pub description: String,
    /// Journey query associated with the issue.
    #[serde(
        rename = "journey_query",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub journey_query: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Name of the issue.
    #[serde(rename = "name")]
    pub name: String,
    /// A representative session illustrating a replay analysis issue.
    #[serde(rename = "representative_session")]
    pub representative_session: crate::datadogV2::model::ReplayAnalysisRepresentativeSession,
    /// Total number of sessions affected by this issue.
    #[serde(rename = "session_count")]
    pub session_count: i64,
    /// Severity level of the issue. Valid values are `high`, `medium`, and `low`.
    #[serde(rename = "severity")]
    pub severity: String,
    /// Timestamp when the issue was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Validation status of the issue.
    #[serde(rename = "validation_verdict")]
    pub validation_verdict: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisIssueDataAttributes {
    pub fn new(
        affected_sessions: Vec<crate::datadogV2::model::ReplayAnalysisAffectedSession>,
        application_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        name: String,
        representative_session: crate::datadogV2::model::ReplayAnalysisRepresentativeSession,
        session_count: i64,
        severity: String,
        updated_at: chrono::DateTime<chrono::Utc>,
        validation_verdict: String,
    ) -> ReplayAnalysisIssueDataAttributes {
        ReplayAnalysisIssueDataAttributes {
            affected_sessions,
            application_id,
            created_at,
            description,
            journey_query: None,
            name,
            representative_session,
            session_count,
            severity,
            updated_at,
            validation_verdict,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn journey_query(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.journey_query = Some(value);
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

impl<'de> Deserialize<'de> for ReplayAnalysisIssueDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisIssueDataAttributesVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisIssueDataAttributesVisitor {
            type Value = ReplayAnalysisIssueDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut affected_sessions: Option<
                    Vec<crate::datadogV2::model::ReplayAnalysisAffectedSession>,
                > = None;
                let mut application_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut journey_query: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut name: Option<String> = None;
                let mut representative_session: Option<
                    crate::datadogV2::model::ReplayAnalysisRepresentativeSession,
                > = None;
                let mut session_count: Option<i64> = None;
                let mut severity: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut validation_verdict: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "affected_sessions" => {
                            affected_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "journey_query" => {
                            journey_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "representative_session" => {
                            representative_session =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count" => {
                            session_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "validation_verdict" => {
                            validation_verdict =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let affected_sessions = affected_sessions
                    .ok_or_else(|| M::Error::missing_field("affected_sessions"))?;
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let representative_session = representative_session
                    .ok_or_else(|| M::Error::missing_field("representative_session"))?;
                let session_count =
                    session_count.ok_or_else(|| M::Error::missing_field("session_count"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let validation_verdict = validation_verdict
                    .ok_or_else(|| M::Error::missing_field("validation_verdict"))?;

                let content = ReplayAnalysisIssueDataAttributes {
                    affected_sessions,
                    application_id,
                    created_at,
                    description,
                    journey_query,
                    name,
                    representative_session,
                    session_count,
                    severity,
                    updated_at,
                    validation_verdict,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisIssueDataAttributesVisitor)
    }
}
