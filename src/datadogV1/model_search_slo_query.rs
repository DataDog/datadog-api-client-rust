// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator", skip_serializing_if = "Option::is_none")]
    pub denominator: String,
    /// Metric names used in the query's numerator and denominator.
This field will return null and will be implemented in the next version of this endpoint.
    #[serde(rename = "metrics", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub metrics: datadog.NullableList[String],
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator", skip_serializing_if = "Option::is_none")]
    pub numerator: String,
}

