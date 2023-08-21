// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageMeasurement {
    /// Type of usage.
    #[serde(rename = "usage_type", skip_serializing_if = "Option::is_none")]
    pub usage_type: String,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub value: Option<Int64>,
}

