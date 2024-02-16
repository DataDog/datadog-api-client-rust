// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pipelines and processors operate on incoming logs,
/// parsing and transforming them into structured attributes for easier querying.
///
/// **Note**: These endpoints are only available for admin users.
/// Make sure to use an application key created by an admin.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsPipeline {
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV1::model::LogsFilter>,
    /// ID of the pipeline.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether or not the pipeline is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Whether or not the pipeline can be edited.
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Name of the pipeline.
    #[serde(rename = "name")]
    pub name: String,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors")]
    pub processors: Option<Vec<crate::datadogV1::model::LogsProcessor>>,
    /// Type of pipeline.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl LogsPipeline {
    pub fn new(name: String) -> LogsPipeline {
        LogsPipeline {
            filter: None,
            id: None,
            is_enabled: None,
            is_read_only: None,
            name,
            processors: None,
            type_: None,
        }
    }

    pub fn filter(&mut self, value: crate::datadogV1::model::LogsFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn is_read_only(&mut self, value: bool) -> &mut Self {
        self.is_read_only = Some(value);
        self
    }

    pub fn processors(&mut self, value: Vec<crate::datadogV1::model::LogsProcessor>) -> &mut Self {
        self.processors = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
