// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Nested Pipelines are pipelines within a pipeline. Use Nested Pipelines to split the processing into two steps.
/// For example, first use a high-level filtering such as team and then a second level of filtering based on the
/// integration, service, or any other tag or attribute.
///
/// A pipeline can contain Nested Pipelines and Processors whereas a Nested Pipeline can only contain Processors.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsPipelineProcessor {
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV1::model::LogsFilter>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors")]
    pub processors: Option<Vec<crate::datadogV1::model::LogsProcessor>>,
    /// Type of logs pipeline processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsPipelineProcessorType,
}

impl LogsPipelineProcessor {
    pub fn new(type_: crate::datadogV1::model::LogsPipelineProcessorType) -> LogsPipelineProcessor {
        LogsPipelineProcessor {
            filter: None,
            is_enabled: None,
            name: None,
            processors: None,
            type_,
        }
    }

    pub fn filter(&mut self, value: crate::datadogV1::model::LogsFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn processors(&mut self, value: Vec<crate::datadogV1::model::LogsProcessor>) -> &mut Self {
        self.processors = Some(value);
        self
    }
}
