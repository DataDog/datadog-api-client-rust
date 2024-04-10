// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the top level pipeline, build, or workflow of your CI.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelineEventPipeline {
    /// Time when the pipeline run finished. It cannot be older than 18 hours in the past from the current time. The time format must be RFC3339.
    #[serde(rename = "end")]
    pub end: chrono::DateTime<chrono::Utc>,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<crate::datadogV2::model::CIAppCIError>>,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
    /// Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git", default, with = "::serde_with::rust::double_option")]
    pub git: Option<Option<crate::datadogV2::model::CIAppGitInfo>>,
    /// Whether or not the pipeline was triggered manually by the user.
    #[serde(
        rename = "is_manual",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_manual: Option<Option<bool>>,
    /// Whether or not the pipeline was resumed after being blocked.
    #[serde(
        rename = "is_resumed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_resumed: Option<Option<bool>>,
    /// Used to distinguish between pipelines, stages, jobs, and steps.
    #[serde(rename = "level")]
    pub level: crate::datadogV2::model::CIAppPipelineEventPipelineLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// Name of the pipeline. All pipeline runs for the builds should have the same name.
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
    /// If the pipeline is triggered as child of another pipeline, this should contain the details of the parent pipeline.
    #[serde(
        rename = "parent_pipeline",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parent_pipeline: Option<Option<crate::datadogV2::model::CIAppPipelineEventParentPipeline>>,
    /// Whether or not the pipeline was a partial retry of a previous attempt. A partial retry is one
    /// which only runs a subset of the original jobs.
    #[serde(rename = "partial_retry")]
    pub partial_retry: bool,
    /// Any ID used in the provider to identify the pipeline run even if it is not unique across retries.
    /// If the `pipeline_id` is unique, then both `unique_id` and `pipeline_id` can be set to the same value.
    #[serde(rename = "pipeline_id")]
    pub pipeline_id: Option<String>,
    /// If the pipeline is a retry, this should contain the details of the previous attempt.
    #[serde(
        rename = "previous_attempt",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub previous_attempt:
        Option<Option<crate::datadogV2::model::CIAppPipelineEventPreviousPipeline>>,
    /// The queue time in milliseconds, if applicable.
    #[serde(
        rename = "queue_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub queue_time: Option<Option<i64>>,
    /// Time when the pipeline run started (it should not include any queue time). The time format must be RFC3339.
    #[serde(rename = "start")]
    pub start: chrono::DateTime<chrono::Utc>,
    /// The final status of the pipeline.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CIAppPipelineEventPipelineStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// UUID of the pipeline run. The ID has to be unique across retries and pipelines,
    /// including partial retries.
    #[serde(rename = "unique_id")]
    pub unique_id: String,
    /// The URL to look at the pipeline in the CI provider UI.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelineEventPipeline {
    pub fn new(
        end: chrono::DateTime<chrono::Utc>,
        level: crate::datadogV2::model::CIAppPipelineEventPipelineLevel,
        name: String,
        partial_retry: bool,
        start: chrono::DateTime<chrono::Utc>,
        status: crate::datadogV2::model::CIAppPipelineEventPipelineStatus,
        unique_id: String,
        url: String,
    ) -> CIAppPipelineEventPipeline {
        CIAppPipelineEventPipeline {
            end,
            error: None,
            git: None,
            is_manual: None,
            is_resumed: None,
            level,
            metrics: None,
            name,
            node: None,
            parameters: None,
            parent_pipeline: None,
            partial_retry,
            pipeline_id: None,
            previous_attempt: None,
            queue_time: None,
            start,
            status,
            tags: None,
            unique_id,
            url,
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: Option<crate::datadogV2::model::CIAppCIError>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn git(mut self, value: Option<crate::datadogV2::model::CIAppGitInfo>) -> Self {
        self.git = Some(value);
        self
    }

    pub fn is_manual(mut self, value: Option<bool>) -> Self {
        self.is_manual = Some(value);
        self
    }

    pub fn is_resumed(mut self, value: Option<bool>) -> Self {
        self.is_resumed = Some(value);
        self
    }

    pub fn metrics(mut self, value: Option<Vec<String>>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn node(mut self, value: Option<crate::datadogV2::model::CIAppHostInfo>) -> Self {
        self.node = Some(value);
        self
    }

    pub fn parameters(mut self, value: Option<std::collections::BTreeMap<String, String>>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn parent_pipeline(
        mut self,
        value: Option<crate::datadogV2::model::CIAppPipelineEventParentPipeline>,
    ) -> Self {
        self.parent_pipeline = Some(value);
        self
    }

    pub fn pipeline_id(mut self, value: String) -> Self {
        self.pipeline_id = Some(value);
        self
    }

    pub fn previous_attempt(
        mut self,
        value: Option<crate::datadogV2::model::CIAppPipelineEventPreviousPipeline>,
    ) -> Self {
        self.previous_attempt = Some(value);
        self
    }

    pub fn queue_time(mut self, value: Option<i64>) -> Self {
        self.queue_time = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CIAppPipelineEventPipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelineEventPipelineVisitor;
        impl<'a> Visitor<'a> for CIAppPipelineEventPipelineVisitor {
            type Value = CIAppPipelineEventPipeline;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut error: Option<Option<crate::datadogV2::model::CIAppCIError>> = None;
                let mut git: Option<Option<crate::datadogV2::model::CIAppGitInfo>> = None;
                let mut is_manual: Option<Option<bool>> = None;
                let mut is_resumed: Option<Option<bool>> = None;
                let mut level: Option<crate::datadogV2::model::CIAppPipelineEventPipelineLevel> =
                    None;
                let mut metrics: Option<Option<Vec<String>>> = None;
                let mut name: Option<String> = None;
                let mut node: Option<Option<crate::datadogV2::model::CIAppHostInfo>> = None;
                let mut parameters: Option<Option<std::collections::BTreeMap<String, String>>> =
                    None;
                let mut parent_pipeline: Option<
                    Option<crate::datadogV2::model::CIAppPipelineEventParentPipeline>,
                > = None;
                let mut partial_retry: Option<bool> = None;
                let mut pipeline_id: Option<String> = None;
                let mut previous_attempt: Option<
                    Option<crate::datadogV2::model::CIAppPipelineEventPreviousPipeline>,
                > = None;
                let mut queue_time: Option<Option<i64>> = None;
                let mut start: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::CIAppPipelineEventPipelineStatus> =
                    None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut unique_id: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git" => {
                            git = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_manual" => {
                            is_manual = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_resumed" => {
                            is_resumed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "level" => {
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _level) = level {
                                match _level {
                                    crate::datadogV2::model::CIAppPipelineEventPipelineLevel::UnparsedObject(_level) => {
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
                        "parent_pipeline" => {
                            parent_pipeline =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "partial_retry" => {
                            partial_retry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_id" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previous_attempt" => {
                            previous_attempt =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queue_time" => {
                            queue_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CIAppPipelineEventPipelineStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unique_id" => {
                            unique_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let level = level.ok_or_else(|| M::Error::missing_field("level"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let partial_retry =
                    partial_retry.ok_or_else(|| M::Error::missing_field("partial_retry"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let unique_id = unique_id.ok_or_else(|| M::Error::missing_field("unique_id"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = CIAppPipelineEventPipeline {
                    end,
                    error,
                    git,
                    is_manual,
                    is_resumed,
                    level,
                    metrics,
                    name,
                    node,
                    parameters,
                    parent_pipeline,
                    partial_retry,
                    pipeline_id,
                    previous_attempt,
                    queue_time,
                    start,
                    status,
                    tags,
                    unique_id,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelineEventPipelineVisitor)
    }
}
