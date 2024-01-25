// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// There are two ways to improve correlation between application traces and logs.
///
///   1. Follow the documentation on [how to inject a trace ID in the application logs](<https://docs.datadoghq.com/tracing/connect_logs_and_traces>)
///   and by default log integrations take care of all the rest of the setup.
///
///   2. Use the Trace remapper processor to define a log attribute as its associated trace ID.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsTraceRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,
    /// Type of logs trace remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsTraceRemapperType,
}

impl LogsTraceRemapper {
    pub fn new(type_: crate::datadogV1::model::LogsTraceRemapperType) -> LogsTraceRemapper {
        LogsTraceRemapper {
            is_enabled: None,
            name: None,
            sources: None,
            type_,
        }
    }
}
