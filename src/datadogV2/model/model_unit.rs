// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the metric unit family, scale factor, name, and short name.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
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

impl Unit {
    pub fn new() -> Unit {
        Unit {
            family: None,
            name: None,
            plural: None,
            scale_factor: None,
            short_name: None,
        }
    }

    pub fn with_family(&mut self, value: String) -> &mut Self {
        self.family = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_plural(&mut self, value: String) -> &mut Self {
        self.plural = Some(value);
        self
    }

    pub fn with_scale_factor(&mut self, value: f64) -> &mut Self {
        self.scale_factor = Some(value);
        self
    }

    pub fn with_short_name(&mut self, value: String) -> &mut Self {
        self.short_name = Some(value);
        self
    }
}
impl Default for Unit {
    fn default() -> Self {
        Self::new()
    }
}
