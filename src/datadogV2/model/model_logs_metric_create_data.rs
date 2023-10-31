// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The new log-based metric properties.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricCreateData {
    /// The object describing the Datadog log-based metric to create.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::LogsMetricCreateAttributes>,
    /// The name of the log-based metric.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the resource. The value should always be logs_metrics.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsMetricType,
}

impl LogsMetricCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::LogsMetricCreateAttributes,
        id: String,
        type_: crate::datadogV2::model::LogsMetricType,
    ) -> LogsMetricCreateData {
        LogsMetricCreateData {
            attributes: Box::new(attributes),
            id,
            type_,
        }
    }
}
