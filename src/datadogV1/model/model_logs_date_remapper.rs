// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// As Datadog receives logs, it timestamps them using the value(s) from any of these default attributes.
///
///   - `timestamp`
///   - `date`
///   - `_timestamp`
///   - `Timestamp`
///   - `eventTime`
///   - `published_date`
///
///   If your logs put their dates in an attribute not in this list,
///   use the log date Remapper Processor to define their date attribute as the official log timestamp.
///   The recognized date formats are ISO8601, UNIX (the milliseconds EPOCH format), and RFC3164.
///
///   **Note:** If your logs don’t contain any of the default attributes
///   and you haven’t defined your own date attribute, Datadog timestamps
///   the logs with the date it received them.
///
///   If multiple log date remapper processors can be applied to a given log,
///   only the first one (according to the pipelines order) is taken into account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsDateRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Type of logs date remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsDateRemapperType,
}

impl LogsDateRemapper {
    pub fn new(
        sources: Vec<String>,
        type_: crate::datadogV1::model::LogsDateRemapperType,
    ) -> LogsDateRemapper {
        LogsDateRemapper {
            is_enabled: None,
            name: None,
            sources,
            type_,
        }
    }

    pub fn with_is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
