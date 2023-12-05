// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An Object of metric units.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetricsSeriesMetadataUnit {
    /// The family of metric unit, for example `bytes` is the family for `kibibyte`, `byte`, and `bit` units.
    #[serde(rename = "family")]
    pub family: Option<String>,
    /// The ID of the metric unit.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The unit of the metric, for instance `byte`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The plural Unit of metric, for instance `bytes`.
    #[serde(rename = "plural", default, with = "::serde_with::rust::double_option")]
    pub plural: Option<Option<String>>,
    /// The scale factor of metric unit, for instance `1.0`.
    #[serde(rename = "scale_factor")]
    pub scale_factor: Option<f64>,
    /// A shorter and abbreviated version of the metric unit, for instance `B`.
    #[serde(
        rename = "short_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub short_name: Option<Option<String>>,
}

impl SLOHistoryMetricsSeriesMetadataUnit {
    pub fn new() -> SLOHistoryMetricsSeriesMetadataUnit {
        SLOHistoryMetricsSeriesMetadataUnit {
            family: None,
            id: None,
            name: None,
            plural: None,
            scale_factor: None,
            short_name: None,
        }
    }
}