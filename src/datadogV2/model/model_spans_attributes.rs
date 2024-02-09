// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all span attributes and their associated values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAttributes {
    /// JSON object of attributes from your span.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// JSON object of custom spans data.
    #[serde(rename = "custom")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// End timestamp of your span.
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: Option<String>,
    /// Name of the environment from where the spans are being sent.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Name of the machine from where the spans are being sent.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The reason why the span was ingested.
    #[serde(rename = "ingestion_reason")]
    pub ingestion_reason: Option<String>,
    /// Id of the span that's parent of this span.
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    /// Unique identifier of the resource.
    #[serde(rename = "resource_hash")]
    pub resource_hash: Option<String>,
    /// The name of the resource.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// The reason why the span was indexed.
    #[serde(rename = "retained_by")]
    pub retained_by: Option<String>,
    /// The name of the application or service generating the span events.
    /// It is used to switch from APM to Logs, so make sure you define the same
    /// value when you use both products.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Whether or not the span was collected as a stand-alone span. Always associated to "single_span" ingestion_reason if true.
    #[serde(rename = "single_span")]
    pub single_span: Option<bool>,
    /// Id of the span.
    #[serde(rename = "span_id")]
    pub span_id: Option<String>,
    /// Start timestamp of your span.
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: Option<String>,
    /// Array of tags associated with your span.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Id of the trace to which the span belongs.
    #[serde(rename = "trace_id")]
    pub trace_id: Option<String>,
    /// The type of the span.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl SpansAttributes {
    pub fn new() -> SpansAttributes {
        SpansAttributes {
            attributes: None,
            custom: None,
            end_timestamp: None,
            env: None,
            host: None,
            ingestion_reason: None,
            parent_id: None,
            resource_hash: None,
            resource_name: None,
            retained_by: None,
            service: None,
            single_span: None,
            span_id: None,
            start_timestamp: None,
            tags: None,
            trace_id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn custom(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.custom = Some(value);
        self
    }

    pub fn end_timestamp(&mut self, value: String) -> &mut Self {
        self.end_timestamp = Some(value);
        self
    }

    pub fn env(&mut self, value: String) -> &mut Self {
        self.env = Some(value);
        self
    }

    pub fn host(&mut self, value: String) -> &mut Self {
        self.host = Some(value);
        self
    }

    pub fn ingestion_reason(&mut self, value: String) -> &mut Self {
        self.ingestion_reason = Some(value);
        self
    }

    pub fn parent_id(&mut self, value: String) -> &mut Self {
        self.parent_id = Some(value);
        self
    }

    pub fn resource_hash(&mut self, value: String) -> &mut Self {
        self.resource_hash = Some(value);
        self
    }

    pub fn resource_name(&mut self, value: String) -> &mut Self {
        self.resource_name = Some(value);
        self
    }

    pub fn retained_by(&mut self, value: String) -> &mut Self {
        self.retained_by = Some(value);
        self
    }

    pub fn service(&mut self, value: String) -> &mut Self {
        self.service = Some(value);
        self
    }

    pub fn single_span(&mut self, value: bool) -> &mut Self {
        self.single_span = Some(value);
        self
    }

    pub fn span_id(&mut self, value: String) -> &mut Self {
        self.span_id = Some(value);
        self
    }

    pub fn start_timestamp(&mut self, value: String) -> &mut Self {
        self.start_timestamp = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn trace_id(&mut self, value: String) -> &mut Self {
        self.trace_id = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SpansAttributes {
    fn default() -> Self {
        Self::new()
    }
}
