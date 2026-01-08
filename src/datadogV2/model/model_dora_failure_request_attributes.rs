// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to create a DORA failure event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORAFailureRequestAttributes {
    /// A list of user-defined tags. The tags must follow the `key:value` pattern. Up to 100 may be added per event.
    #[serde(
        rename = "custom_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_tags: Option<Option<Vec<String>>>,
    /// Environment name that was impacted by the failure.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Unix timestamp when the failure finished. It must be in nanoseconds, milliseconds, or seconds.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// Git info for DORA Metrics events.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV2::model::DORAGitInfo>,
    /// Failure ID. Must be 16-128 characters and contain only alphanumeric characters, hyphens, underscores, periods, and colons (a-z, A-Z, 0-9, -, _, ., :).
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Failure name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Service names impacted by the failure. If possible, use names registered in the Service Catalog. Required when the team field is not provided.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Failure severity.
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    /// Unix timestamp when the failure started. It must be in nanoseconds, milliseconds, or seconds.
    #[serde(rename = "started_at")]
    pub started_at: i64,
    /// Name of the team owning the services impacted. If possible, use team handles registered in Datadog. Required when the services field is not provided.
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

impl DORAFailureRequestAttributes {
    pub fn new(started_at: i64) -> DORAFailureRequestAttributes {
        DORAFailureRequestAttributes {
            custom_tags: None,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DORAFailureRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORAFailureRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DORAFailureRequestAttributesVisitor {
            type Value = DORAFailureRequestAttributes;

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
                let mut name: Option<String> = None;
                let mut services: Option<Vec<String>> = None;
                let mut severity: Option<String> = None;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;

                let content = DORAFailureRequestAttributes {
                    custom_tags,
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
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORAFailureRequestAttributesVisitor)
    }
}
