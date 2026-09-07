// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the deployment event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORADeploymentObjectAttributes {
    /// AI-assisted development metrics aggregated across the commits and pull requests included in the deployment.
    #[serde(rename = "ai")]
    pub ai: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Averaged DORA and delivery metrics computed across the commits and pull requests included in the deployment.
    #[serde(rename = "averaged_metrics")]
    pub averaged_metrics: Option<crate::datadogV2::model::DORADeploymentAveragedMetrics>,
    /// Whether the deployment is flagged as a change failure.
    #[serde(rename = "change_failure")]
    pub change_failure: Option<bool>,
    /// The list of commits included in the deployment.
    #[serde(rename = "commits")]
    pub commits: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// The time when the deployment event was recorded.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A map of custom metadata associated with the deployment.
    #[serde(rename = "custom")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern. Up to 100 may be added per event.
    #[serde(
        rename = "custom_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_tags: Option<Option<Vec<String>>>,
    /// The type of the deployment.
    #[serde(rename = "deployment_type")]
    pub deployment_type: Option<String>,
    /// The duration of the deployment.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Environment name to where the service was deployed.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// The time when the deployment finished.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Git info returned by DORA Metrics events.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV2::model::DORAGitInfoResponse>,
    /// The number of commits associated with the deployment.
    #[serde(rename = "number_of_commits")]
    pub number_of_commits: Option<i64>,
    /// The number of pull requests associated with the deployment.
    #[serde(rename = "number_of_pull_requests")]
    pub number_of_pull_requests: Option<i64>,
    /// The list of pull requests included in the deployment.
    #[serde(rename = "pull_requests")]
    pub pull_requests: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// The recovery time, in seconds, for a deployment flagged as a change failure.
    #[serde(rename = "recovery_time_sec")]
    pub recovery_time_sec: Option<i64>,
    /// Remediation details for a deployment that was flagged as a change failure.
    #[serde(rename = "remediation")]
    pub remediation: Option<crate::datadogV2::model::DORADeploymentRemediation>,
    /// Service name.
    #[serde(rename = "service")]
    pub service: String,
    /// The source of the deployment event.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// The time when the deployment started.
    #[serde(rename = "started_at")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Name of the team owning the deployed service.
    #[serde(rename = "team")]
    pub team: Option<String>,
    /// Version to correlate with APM Deployment Tracking.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORADeploymentObjectAttributes {
    pub fn new(
        service: String,
        started_at: chrono::DateTime<chrono::Utc>,
    ) -> DORADeploymentObjectAttributes {
        DORADeploymentObjectAttributes {
            ai: None,
            averaged_metrics: None,
            change_failure: None,
            commits: None,
            created_at: None,
            custom: None,
            custom_tags: None,
            deployment_type: None,
            duration: None,
            env: None,
            finished_at: None,
            git: None,
            number_of_commits: None,
            number_of_pull_requests: None,
            pull_requests: None,
            recovery_time_sec: None,
            remediation: None,
            service,
            source: None,
            started_at,
            team: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ai(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.ai = Some(value);
        self
    }

    pub fn averaged_metrics(
        mut self,
        value: crate::datadogV2::model::DORADeploymentAveragedMetrics,
    ) -> Self {
        self.averaged_metrics = Some(value);
        self
    }

    pub fn change_failure(mut self, value: bool) -> Self {
        self.change_failure = Some(value);
        self
    }

    pub fn commits(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.commits = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn custom(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn custom_tags(mut self, value: Option<Vec<String>>) -> Self {
        self.custom_tags = Some(value);
        self
    }

    pub fn deployment_type(mut self, value: String) -> Self {
        self.deployment_type = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn finished_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn git(mut self, value: crate::datadogV2::model::DORAGitInfoResponse) -> Self {
        self.git = Some(value);
        self
    }

    pub fn number_of_commits(mut self, value: i64) -> Self {
        self.number_of_commits = Some(value);
        self
    }

    pub fn number_of_pull_requests(mut self, value: i64) -> Self {
        self.number_of_pull_requests = Some(value);
        self
    }

    pub fn pull_requests(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.pull_requests = Some(value);
        self
    }

    pub fn recovery_time_sec(mut self, value: i64) -> Self {
        self.recovery_time_sec = Some(value);
        self
    }

    pub fn remediation(
        mut self,
        value: crate::datadogV2::model::DORADeploymentRemediation,
    ) -> Self {
        self.remediation = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DORADeploymentObjectAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORADeploymentObjectAttributesVisitor;
        impl<'a> Visitor<'a> for DORADeploymentObjectAttributesVisitor {
            type Value = DORADeploymentObjectAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ai: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut averaged_metrics: Option<
                    crate::datadogV2::model::DORADeploymentAveragedMetrics,
                > = None;
                let mut change_failure: Option<bool> = None;
                let mut commits: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut custom: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut custom_tags: Option<Option<Vec<String>>> = None;
                let mut deployment_type: Option<String> = None;
                let mut duration: Option<i64> = None;
                let mut env: Option<String> = None;
                let mut finished_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut git: Option<crate::datadogV2::model::DORAGitInfoResponse> = None;
                let mut number_of_commits: Option<i64> = None;
                let mut number_of_pull_requests: Option<i64> = None;
                let mut pull_requests: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut recovery_time_sec: Option<i64> = None;
                let mut remediation: Option<crate::datadogV2::model::DORADeploymentRemediation> =
                    None;
                let mut service: Option<String> = None;
                let mut source: Option<String> = None;
                let mut started_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut team: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ai" => {
                            if v.is_null() {
                                continue;
                            }
                            ai = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "averaged_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            averaged_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_failure" => {
                            if v.is_null() {
                                continue;
                            }
                            change_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commits" => {
                            if v.is_null() {
                                continue;
                            }
                            commits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_tags" => {
                            custom_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deployment_type" => {
                            if v.is_null() {
                                continue;
                            }
                            deployment_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "number_of_commits" => {
                            if v.is_null() {
                                continue;
                            }
                            number_of_commits =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number_of_pull_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            number_of_pull_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pull_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            pull_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recovery_time_sec" => {
                            if v.is_null() {
                                continue;
                            }
                            recovery_time_sec =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remediation" => {
                            if v.is_null() {
                                continue;
                            }
                            remediation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;

                let content = DORADeploymentObjectAttributes {
                    ai,
                    averaged_metrics,
                    change_failure,
                    commits,
                    created_at,
                    custom,
                    custom_tags,
                    deployment_type,
                    duration,
                    env,
                    finished_at,
                    git,
                    number_of_commits,
                    number_of_pull_requests,
                    pull_requests,
                    recovery_time_sec,
                    remediation,
                    service,
                    source,
                    started_at,
                    team,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORADeploymentObjectAttributesVisitor)
    }
}
