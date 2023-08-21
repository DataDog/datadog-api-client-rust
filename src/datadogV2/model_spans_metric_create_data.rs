// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricCreateData {
    /// The object describing the Datadog span-based metric to create.
    #[serde(rename = "attributes")]
    pub attributes: SpansMetricCreateAttributes,
    /// The name of the span-based metric.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The type of resource. The value should always be spans_metrics.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SpansMetricType,
}

