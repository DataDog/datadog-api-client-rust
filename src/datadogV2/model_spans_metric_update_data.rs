// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricUpdateData {
    /// The span-based metric properties that will be updated.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SpansMetricUpdateAttributes,
    /// The type of resource. The value should always be spans_metrics.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SpansMetricType,
}

