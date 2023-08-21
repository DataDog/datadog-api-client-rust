// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLevelObjectiveQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator", skip_serializing_if = "Option::is_none")]
    pub denominator: String,
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator", skip_serializing_if = "Option::is_none")]
    pub numerator: String,
}

