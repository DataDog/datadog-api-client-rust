// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCoreWebVitals {
    /// Cumulative Layout Shift.
    #[serde(rename = "cls", skip_serializing_if = "Option::is_none")]
    pub cls: f64,
    /// Largest Contentful Paint in milliseconds.
    #[serde(rename = "lcp", skip_serializing_if = "Option::is_none")]
    pub lcp: f64,
    /// URL attached to the metrics.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

