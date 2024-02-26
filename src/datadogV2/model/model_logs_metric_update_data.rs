// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The new log-based metric properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricUpdateData {
    /// The log-based metric properties that will be updated.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::LogsMetricUpdateAttributes,
    /// The type of the resource. The value should always be logs_metrics.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsMetricType,
}

impl LogsMetricUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::LogsMetricUpdateAttributes,
        type_: crate::datadogV2::model::LogsMetricType,
    ) -> LogsMetricUpdateData {
        LogsMetricUpdateData { attributes, type_ }
    }
}
