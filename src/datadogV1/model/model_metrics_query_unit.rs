// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the metric unit family, scale factor, name, and short name.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryUnit {
    /// Unit family, allows for conversion between units of the same family, for scaling.
    #[serde(rename = "family")]
    pub family: Option<String>,
    /// Unit name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Plural form of the unit name.
    #[serde(rename = "plural")]
    pub plural: Option<String>,
    /// Factor for scaling between units of the same family.
    #[serde(rename = "scale_factor")]
    pub scale_factor: Option<f64>,
    /// Abbreviation of the unit.
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
}

impl MetricsQueryUnit {
    pub fn new() -> MetricsQueryUnit {
        MetricsQueryUnit {
            family: None,
            name: None,
            plural: None,
            scale_factor: None,
            short_name: None,
        }
    }
}
