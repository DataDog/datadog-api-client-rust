// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The message is a key attribute in Datadog.
/// It is displayed in the message column of the Log Explorer and you can do full string search on it.
/// Use this Processor to define one or more attributes as the official log message.
///
/// **Note:** If multiple log message remapper processors can be applied to a given log,
/// only the first one (according to the pipeline order) is taken into account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMessageRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Type of logs message remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsMessageRemapperType,
}

impl LogsMessageRemapper {
    pub fn new(
        sources: Vec<String>,
        type_: crate::datadogV1::model::LogsMessageRemapperType,
    ) -> LogsMessageRemapper {
        LogsMessageRemapper {
            is_enabled: None,
            name: None,
            sources,
            type_,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}
