// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of a CI job.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelineEventJob {
    /// A list of job IDs that this job depends on.
    #[serde(
        rename = "dependencies",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dependencies: Option<Option<Vec<String>>>,
    /// Time when the job run finished. The time format must be RFC3339.
    #[serde(rename = "end")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<crate::datadogV2::model::CIAppCIError>>,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
    /// Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git", default, with = "::serde_with::rust::double_option")]
    pub git: Option<Option<crate::datadogV2::model::CIAppGitInfo>>,
    /// The UUID for the job. It has to be unique within each pipeline execution.
    #[serde(rename = "id")]
    pub id: String,
    /// Used to distinguish between pipelines, stages, jobs, and steps.
    #[serde(rename = "level")]
    pub level: crate::datadogV2::model::CIAppPipelineEventJobLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// The name for the job.
    #[serde(rename = "name")]
    pub name: String,
    /// Contains information of the host running the pipeline, stage, job, or step.
    #[serde(rename = "node", default, with = "::serde_with::rust::double_option")]
    pub node: Option<Option<crate::datadogV2::model::CIAppHostInfo>>,
    /// A map of key-value parameters or environment variables that were defined for the pipeline.
    #[serde(
        rename = "parameters",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parameters: Option<Option<std::collections::BTreeMap<String, String>>>,
    /// The parent pipeline name.
    #[serde(rename = "pipeline_name")]
    pub pipeline_name: String,
    /// The parent pipeline UUID.
    #[serde(rename = "pipeline_unique_id")]
    pub pipeline_unique_id: String,
    /// The queue time in milliseconds, if applicable.
    #[serde(
        rename = "queue_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub queue_time: Option<Option<i64>>,
    /// The parent stage UUID (if applicable).
    #[serde(
        rename = "stage_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub stage_id: Option<Option<String>>,
    /// The parent stage name (if applicable).
    #[serde(
        rename = "stage_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub stage_name: Option<Option<String>>,
    /// Time when the job run instance started (it should not include any queue time). The time format must be RFC3339.
    #[serde(rename = "start")]
    pub start: String,
    /// The final status of the job.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CIAppPipelineEventJobStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// The URL to look at the job in the CI provider UI.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelineEventJob {
    pub fn new(
        end: String,
        id: String,
        level: crate::datadogV2::model::CIAppPipelineEventJobLevel,
        name: String,
        pipeline_name: String,
        pipeline_unique_id: String,
        start: String,
        status: crate::datadogV2::model::CIAppPipelineEventJobStatus,
        url: String,
    ) -> CIAppPipelineEventJob {
        CIAppPipelineEventJob {
            dependencies: None,
            end,
            error: None,
            git: None,
            id,
            level,
            metrics: None,
            name,
            node: None,
            parameters: None,
            pipeline_name,
            pipeline_unique_id,
            queue_time: None,
            stage_id: None,
            stage_name: None,
            start,
            status,
            tags: None,
            url,
            _unparsed: false,
        }
    }

    pub fn dependencies(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.dependencies = Some(value);
        self
    }

    pub fn error(&mut self, value: Option<crate::datadogV2::model::CIAppCIError>) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn git(&mut self, value: Option<crate::datadogV2::model::CIAppGitInfo>) -> &mut Self {
        self.git = Some(value);
        self
    }

    pub fn metrics(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.metrics = Some(value);
        self
    }

    pub fn node(&mut self, value: Option<crate::datadogV2::model::CIAppHostInfo>) -> &mut Self {
        self.node = Some(value);
        self
    }

    pub fn parameters(
        &mut self,
        value: Option<std::collections::BTreeMap<String, String>>,
    ) -> &mut Self {
        self.parameters = Some(value);
        self
    }

    pub fn queue_time(&mut self, value: Option<i64>) -> &mut Self {
        self.queue_time = Some(value);
        self
    }

    pub fn stage_id(&mut self, value: Option<String>) -> &mut Self {
        self.stage_id = Some(value);
        self
    }

    pub fn stage_name(&mut self, value: Option<String>) -> &mut Self {
        self.stage_name = Some(value);
        self
    }

    pub fn tags(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CIAppPipelineEventJob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelineEventJobVisitor;
        impl<'a> Visitor<'a> for CIAppPipelineEventJobVisitor {
            type Value = CIAppPipelineEventJob;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dependencies: Option<Option<Vec<String>>> = None;
                let mut end: Option<String> = None;
                let mut error: Option<Option<crate::datadogV2::model::CIAppCIError>> = None;
                let mut git: Option<Option<crate::datadogV2::model::CIAppGitInfo>> = None;
                let mut id: Option<String> = None;
                let mut level: Option<crate::datadogV2::model::CIAppPipelineEventJobLevel> = None;
                let mut metrics: Option<Option<Vec<String>>> = None;
                let mut name: Option<String> = None;
                let mut node: Option<Option<crate::datadogV2::model::CIAppHostInfo>> = None;
                let mut parameters: Option<Option<std::collections::BTreeMap<String, String>>> =
                    None;
                let mut pipeline_name: Option<String> = None;
                let mut pipeline_unique_id: Option<String> = None;
                let mut queue_time: Option<Option<i64>> = None;
                let mut stage_id: Option<Option<String>> = None;
                let mut stage_name: Option<Option<String>> = None;
                let mut start: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::CIAppPipelineEventJobStatus> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dependencies" => {
                            dependencies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git" => {
                            git = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "level" => {
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _level) = level {
                                match _level {
                                    crate::datadogV2::model::CIAppPipelineEventJobLevel::UnparsedObject(_level) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "metrics" => {
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "node" => {
                            node = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parameters" => {
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_name" => {
                            pipeline_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_unique_id" => {
                            pipeline_unique_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queue_time" => {
                            queue_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stage_id" => {
                            stage_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stage_name" => {
                            stage_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CIAppPipelineEventJobStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let level = level.ok_or_else(|| M::Error::missing_field("level"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let pipeline_name =
                    pipeline_name.ok_or_else(|| M::Error::missing_field("pipeline_name"))?;
                let pipeline_unique_id = pipeline_unique_id
                    .ok_or_else(|| M::Error::missing_field("pipeline_unique_id"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = CIAppPipelineEventJob {
                    dependencies,
                    end,
                    error,
                    git,
                    id,
                    level,
                    metrics,
                    name,
                    node,
                    parameters,
                    pipeline_name,
                    pipeline_unique_id,
                    queue_time,
                    stage_id,
                    stage_name,
                    start,
                    status,
                    tags,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelineEventJobVisitor)
    }
}
