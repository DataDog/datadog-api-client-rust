// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The log-based metric properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricResponseData {
    /// The object describing a Datadog log-based metric.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::LogsMetricResponseAttributes>,
    /// The name of the log-based metric.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the resource. The value should always be logs_metrics.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::LogsMetricType>,
}

impl LogsMetricResponseData {
    pub fn new() -> LogsMetricResponseData {
        LogsMetricResponseData {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::LogsMetricResponseAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::LogsMetricType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for LogsMetricResponseData {
    fn default() -> Self {
        Self::new()
    }
}
