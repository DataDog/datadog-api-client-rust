// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use this Processor if you want to assign some attributes as the official status.
///
/// Each incoming status value is mapped as follows.
///
///   - Integers from 0 to 7 map to the Syslog severity standards
///   - Strings beginning with `emerg` or f (case-insensitive) map to `emerg` (0)
///   - Strings beginning with `a` (case-insensitive) map to `alert` (1)
///   - Strings beginning with `c` (case-insensitive) map to `critical` (2)
///   - Strings beginning with `err` (case-insensitive) map to `error` (3)
///   - Strings beginning with `w` (case-insensitive) map to `warning` (4)
///   - Strings beginning with `n` (case-insensitive) map to `notice` (5)
///   - Strings beginning with `i` (case-insensitive) map to `info` (6)
///   - Strings beginning with `d`, `trace` or `verbose` (case-insensitive) map to `debug` (7)
///   - Strings beginning with `o` or matching `OK` or `Success` (case-insensitive) map to OK
///   - All others map to `info` (6)
///
///   **Note:** If multiple log status remapper processors can be applied to a given log,
///   only the first one (according to the pipelines order) is taken into account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsStatusRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Type of logs status remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsStatusRemapperType,
}

impl LogsStatusRemapper {
    pub fn new(
        sources: Vec<String>,
        type_: crate::datadogV1::model::LogsStatusRemapperType,
    ) -> LogsStatusRemapper {
        LogsStatusRemapper {
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
