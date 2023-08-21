// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinition {
    /// A list of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// Array of one request object to display in the widget. The request must contain a `group-by` tag whose value is a country ISO code.

See the [Request JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/request_json)
for information about building the `REQUEST_SCHEMA`.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<GeomapWidgetRequest>,
    /// The style to apply to the widget.
    #[serde(rename = "style")]
    pub style: GeomapWidgetDefinitionStyle,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// The title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// The size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the geomap widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: GeomapWidgetDefinitionType,
    /// The view of the world that the map should render.
    #[serde(rename = "view")]
    pub view: GeomapWidgetDefinitionView,
}

