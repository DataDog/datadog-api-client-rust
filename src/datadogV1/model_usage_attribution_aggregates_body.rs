// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionAggregatesBody {
    /// The aggregate type.
    #[serde(rename = "agg_type", skip_serializing_if = "Option::is_none")]
    pub agg_type: String,
    /// The field.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: String,
    /// The value for a given field.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: f64,
}

