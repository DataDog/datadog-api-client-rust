// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAttributes {
    /// JSON object of attributes from your span.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: map[string]interface{},
    /// JSON object of custom spans data.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: map[string]interface{},
    /// End timestamp of your span.
    #[serde(rename = "end_timestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: String,
    /// Name of the environment from where the spans are being sent.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: String,
    /// Name of the machine from where the spans are being sent.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: String,
    /// The reason why the span was ingested.
    #[serde(rename = "ingestion_reason", skip_serializing_if = "Option::is_none")]
    pub ingestion_reason: String,
    /// Id of the span that's parent of this span.
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    pub parent_id: String,
    /// Unique identifier of the resource.
    #[serde(rename = "resource_hash", skip_serializing_if = "Option::is_none")]
    pub resource_hash: String,
    /// The name of the resource.
    #[serde(rename = "resource_name", skip_serializing_if = "Option::is_none")]
    pub resource_name: String,
    /// The reason why the span was indexed.
    #[serde(rename = "retained_by", skip_serializing_if = "Option::is_none")]
    pub retained_by: String,
    /// The name of the application or service generating the span events.
It is used to switch from APM to Logs, so make sure you define the same
value when you use both products.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// Whether or not the span was collected as a stand-alone span. Always associated to "single_span" ingestion_reason if true.
    #[serde(rename = "single_span", skip_serializing_if = "Option::is_none")]
    pub single_span: bool,
    /// Id of the span.
    #[serde(rename = "span_id", skip_serializing_if = "Option::is_none")]
    pub span_id: String,
    /// Start timestamp of your span.
    #[serde(rename = "start_timestamp", skip_serializing_if = "Option::is_none")]
    pub start_timestamp: String,
    /// Array of tags associated with your span.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Id of the trace to which the span belongs.
    #[serde(rename = "trace_id", skip_serializing_if = "Option::is_none")]
    pub trace_id: String,
    /// The type of the span.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

