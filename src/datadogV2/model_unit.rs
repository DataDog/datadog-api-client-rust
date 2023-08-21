// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    /// Unit family, allows for conversion between units of the same family, for scaling.
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: String,
    /// Unit name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Plural form of the unit name.
    #[serde(rename = "plural", skip_serializing_if = "Option::is_none")]
    pub plural: String,
    /// Factor for scaling between units of the same family.
    #[serde(rename = "scale_factor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: f64,
    /// Abbreviation of the unit.
    #[serde(rename = "short_name", skip_serializing_if = "Option::is_none")]
    pub short_name: String,
}

