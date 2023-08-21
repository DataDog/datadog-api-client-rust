// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatMapWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query", skip_serializing_if = "Option::is_none")]
    pub apm_query: LogQueryDefinition,
    /// The event query.
    #[serde(rename = "event_query")]
    pub event_query: EventQueryDefinition,
    /// The log query.
    #[serde(rename = "log_query", skip_serializing_if = "Option::is_none")]
    pub log_query: LogQueryDefinition,
    /// The log query.
    #[serde(rename = "network_query", skip_serializing_if = "Option::is_none")]
    pub network_query: LogQueryDefinition,
    /// The process query to use in the widget.
    #[serde(rename = "process_query")]
    pub process_query: ProcessQueryDefinition,
    /// The log query.
    #[serde(rename = "profile_metrics_query", skip_serializing_if = "Option::is_none")]
    pub profile_metrics_query: LogQueryDefinition,
    /// Widget query.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: String,
    /// The log query.
    #[serde(rename = "rum_query", skip_serializing_if = "Option::is_none")]
    pub rum_query: LogQueryDefinition,
    /// The log query.
    #[serde(rename = "security_query", skip_serializing_if = "Option::is_none")]
    pub security_query: LogQueryDefinition,
    /// Widget style definition.
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: WidgetStyle,
}

