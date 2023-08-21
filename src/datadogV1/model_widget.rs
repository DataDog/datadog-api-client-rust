// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Widget {
    /// [Definition of the widget](https://docs.datadoghq.com/dashboards/widgets/).
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: WidgetDefinition,
    /// ID of the widget.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// The layout for a widget on a `free` or **new dashboard layout** dashboard.
    #[serde(rename = "layout")]
    pub layout: WidgetLayout,
}

