// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricEstimate {
    /// Object containing the definition of a metric estimate attribute.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: MetricEstimateAttributes,
    /// The metric name for this resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The metric estimate resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MetricEstimateResourceType,
}

