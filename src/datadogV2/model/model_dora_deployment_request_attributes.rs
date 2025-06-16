// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to create a DORA deployment event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORADeploymentRequestAttributes {
    /// A list of user-defined tags. The tags must follow the `key:value` pattern. Up to 100 may be added per event.
    #[serde(
        rename = "custom_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_tags: Option<Option<Vec<String>>>,
    /// Environment name to where the service was deployed.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Unix timestamp when the deployment finished. It must be in nanoseconds, milliseconds, or seconds, and it should not be older than 1 hour.
    #[serde(rename = "finished_at")]
    pub finished_at: i64,
    /// Git info for DORA Metrics events.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV2::model::DORAGitInfo>,
    /// Deployment ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Service name.
    #[serde(rename = "service")]
    pub service: String,
    /// Unix timestamp when the deployment started. It must be in nanoseconds, milliseconds, or seconds.
    #[serde(rename = "started_at")]
    pub started_at: i64,
    /// Name of the team owning the deployed service. If not provided, this is automatically populated with the team associated with the service in the Service Catalog.
    #[serde(rename = "team")]
    pub team: Option<String>,
    /// Version to correlate with [APM Deployment Tracking](<https://docs.datadoghq.com/tracing/services/deployment_tracking/>).
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORADeploymentRequestAttributes {
    pub fn new(
        finished_at: i64,
        service: String,
        started_at: i64,
    ) -> DORADeploymentRequestAttributes {
        DORADeploymentRequestAttributes {
            custom_tags: None,
            env: None,
            finished_at,
            git: None,
            id: None,
            service,
            started_at,
            team: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_tags(mut self, value: Option<Vec<String>>) -> Self {
        self.custom_tags = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
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

impl<'de> Deserialize<'de> for DORADeploymentRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORADeploymentRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DORADeploymentRequestAttributesVisitor {
            type Value = DORADeploymentRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_tags: Option<Option<Vec<String>>> = None;
                let mut env: Option<String> = None;
                let mut finished_at: Option<i64> = None;
                let mut git: Option<crate::datadogV2::model::DORAGitInfo> = None;
                let mut id: Option<String> = None;
                let mut service: Option<String> = None;
                let mut started_at: Option<i64> = None;
                let mut team: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_tags" => {
                            custom_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finished_at" => {
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
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let finished_at =
                    finished_at.ok_or_else(|| M::Error::missing_field("finished_at"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;

                let content = DORADeploymentRequestAttributes {
                    custom_tags,
                    env,
                    finished_at,
                    git,
                    id,
                    service,
                    started_at,
                    team,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORADeploymentRequestAttributesVisitor)
    }
}
