// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsQueryCompute {
    /// The aggregation method.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: String,
    /// Facet name.
    #[serde(rename = "facet", skip_serializing_if = "Option::is_none")]
    pub facet: String,
    /// Define a time interval in seconds.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
}

