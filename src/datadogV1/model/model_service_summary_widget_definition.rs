// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The service summary displays the graphs of a chosen service in your screenboard. Only available on FREE layout dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSummaryWidgetDefinition {
    /// Number of columns to display.
    #[serde(rename = "display_format")]
    pub display_format: Option<crate::datadogV1::model::WidgetServiceSummaryDisplayFormat>,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// APM service.
    #[serde(rename = "service")]
    pub service: String,
    /// Whether to show the latency breakdown or not.
    #[serde(rename = "show_breakdown")]
    pub show_breakdown: Option<bool>,
    /// Whether to show the latency distribution or not.
    #[serde(rename = "show_distribution")]
    pub show_distribution: Option<bool>,
    /// Whether to show the error metrics or not.
    #[serde(rename = "show_errors")]
    pub show_errors: Option<bool>,
    /// Whether to show the hits metrics or not.
    #[serde(rename = "show_hits")]
    pub show_hits: Option<bool>,
    /// Whether to show the latency metrics or not.
    #[serde(rename = "show_latency")]
    pub show_latency: Option<bool>,
    /// Whether to show the resource list or not.
    #[serde(rename = "show_resource_list")]
    pub show_resource_list: Option<bool>,
    /// Size of the widget.
    #[serde(rename = "size_format")]
    pub size_format: Option<crate::datadogV1::model::WidgetSizeFormat>,
    /// APM span name.
    #[serde(rename = "span_name")]
    pub span_name: String,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<Box<crate::datadogV1::model::WidgetTime>>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the service summary widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ServiceSummaryWidgetDefinitionType,
}

impl ServiceSummaryWidgetDefinition {
    pub fn new(
        env: String,
        service: String,
        span_name: String,
        type_: crate::datadogV1::model::ServiceSummaryWidgetDefinitionType,
    ) -> ServiceSummaryWidgetDefinition {
        ServiceSummaryWidgetDefinition {
            display_format: None,
            env,
            service,
            show_breakdown: None,
            show_distribution: None,
            show_errors: None,
            show_hits: None,
            show_latency: None,
            show_resource_list: None,
            size_format: None,
            span_name,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}
