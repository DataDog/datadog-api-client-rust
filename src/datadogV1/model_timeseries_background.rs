// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesBackground {
    /// Timeseries is made using an area or bars.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: TimeseriesBackgroundType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: WidgetAxis,
}

