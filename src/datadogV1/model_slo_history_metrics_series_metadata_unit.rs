// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetricsSeriesMetadataUnit {
    /// The family of metric unit, for example `bytes` is the family for `kibibyte`, `byte`, and `bit` units.
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: String,
    /// The ID of the metric unit.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// The unit of the metric, for instance `byte`.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The plural Unit of metric, for instance `bytes`.
    #[serde(rename = "plural", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub plural: Option<String>,
    /// The scale factor of metric unit, for instance `1.0`.
    #[serde(rename = "scale_factor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: f64,
    /// A shorter and abbreviated version of the metric unit, for instance `B`.
    #[serde(rename = "short_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub short_name: Option<String>,
}

