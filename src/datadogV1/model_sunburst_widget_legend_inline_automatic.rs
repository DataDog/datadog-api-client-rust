// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SunburstWidgetLegendInlineAutomatic {
    /// Whether to hide the percentages of the groups.
    #[serde(rename = "hide_percent", skip_serializing_if = "Option::is_none")]
    pub hide_percent: bool,
    /// Whether to hide the values of the groups.
    #[serde(rename = "hide_value", skip_serializing_if = "Option::is_none")]
    pub hide_value: bool,
    /// Whether to show the legend inline or let it be automatically generated.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SunburstWidgetLegendInlineAutomaticType,
}

