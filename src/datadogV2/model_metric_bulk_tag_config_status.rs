// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigStatus {
    /// Optional attributes for the status of a bulk tag configuration request.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: MetricBulkTagConfigStatusAttributes,
    /// A text prefix to match against metric names.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The metric bulk configure tags resource.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MetricBulkConfigureTagsType,
}

