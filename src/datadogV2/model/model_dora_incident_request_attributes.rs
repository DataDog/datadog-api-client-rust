// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to create a DORA incident event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORAIncidentRequestAttributes {
    /// Environment name that was impacted by the incident.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Unix timestamp in nanoseconds when the incident finished. It should not be older than 3 hours.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// Git info for DORA Metrics events.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV2::model::DORAGitInfo>,
    /// Incident ID. Required to update a previously sent incident.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Incident name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Service names impacted by the incident. If possible, use names registered in the Service Catalog. Required when the team field is not provided.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Incident severity.
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    /// Unix timestamp in nanoseconds when the incident started.
    #[serde(rename = "started_at")]
    pub started_at: i64,
    /// Name of the team owning the services impacted. If possible, use team handles registered in Datadog. Required when the services field is not provided.
    #[serde(rename = "team")]
    pub team: Option<String>,
    /// Version to correlate with [APM Deployment Tracking](<https://docs.datadoghq.com/tracing/services/deployment_tracking/>).
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORAIncidentRequestAttributes {
    pub fn new(started_at: i64) -> DORAIncidentRequestAttributes {
        DORAIncidentRequestAttributes {
            env: None,
            finished_at: None,
            git: None,
            id: None,
            name: None,
            services: None,
            severity: None,
            started_at,
            team: None,
            version: None,
            _unparsed: false,
        }
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn finished_at(mut self, value: i64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn git(mut self, value: crate::datadogV2::model::DORAGitInfo) -> Self {
        self.git = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn severity(mut self, value: String) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn team(mut self, value: String) -> Self {
        self.team = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DORAIncidentRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORAIncidentRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DORAIncidentRequestAttributesVisitor {
            type Value = DORAIncidentRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut env: Option<String> = None;
                let mut finished_at: Option<i64> = None;
                let mut git: Option<crate::datadogV2::model::DORAGitInfo> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut services: Option<Vec<String>> = None;
                let mut severity: Option<String> = None;
                let mut started_at: Option<i64> = None;
                let mut team: Option<String> = None;
                let mut version: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finished_at" => {
                            if v.is_null() {
                                continue;
                            }
                            finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git" => {
                            if v.is_null() {
                                continue;
                            }
                            git = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            if v.is_null() {
                                continue;
                            }
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;

                let content = DORAIncidentRequestAttributes {
                    env,
                    finished_at,
                    git,
                    id,
                    name,
                    services,
                    severity,
                    started_at,
                    team,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORAIncidentRequestAttributesVisitor)
    }
}
