// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON object containing all span attributes and their associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansAttributes {
    /// JSON object of attributes from your span.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// JSON object of custom spans data.
    #[serde(rename = "custom")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// End timestamp of your span.
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: Option<chrono::DateTime<chrono::Utc>>,
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
    pub start_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Array of tags associated with your span.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Id of the trace to which the span belongs.
    #[serde(rename = "trace_id")]
    pub trace_id: Option<String>,
    /// The type of the span.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn custom(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn end_timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_timestamp = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn ingestion_reason(mut self, value: String) -> Self {
        self.ingestion_reason = Some(value);
        self
    }

    pub fn parent_id(mut self, value: String) -> Self {
        self.parent_id = Some(value);
        self
    }

    pub fn resource_hash(mut self, value: String) -> Self {
        self.resource_hash = Some(value);
        self
    }

    pub fn resource_name(mut self, value: String) -> Self {
        self.resource_name = Some(value);
        self
    }

    pub fn retained_by(mut self, value: String) -> Self {
        self.retained_by = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn single_span(mut self, value: bool) -> Self {
        self.single_span = Some(value);
        self
    }

    pub fn span_id(mut self, value: String) -> Self {
        self.span_id = Some(value);
        self
    }

    pub fn start_timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_timestamp = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn trace_id(mut self, value: String) -> Self {
        self.trace_id = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SpansAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansAttributesVisitor;
        impl<'a> Visitor<'a> for SpansAttributesVisitor {
            type Value = SpansAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut custom: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut end_timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut env: Option<String> = None;
                let mut host: Option<String> = None;
                let mut ingestion_reason: Option<String> = None;
                let mut parent_id: Option<String> = None;
                let mut resource_hash: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut retained_by: Option<String> = None;
                let mut service: Option<String> = None;
                let mut single_span: Option<bool> = None;
                let mut span_id: Option<String> = None;
                let mut start_timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut trace_id: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            end_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingestion_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            ingestion_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_id" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_hash" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_hash =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retained_by" => {
                            if v.is_null() {
                                continue;
                            }
                            retained_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "single_span" => {
                            if v.is_null() {
                                continue;
                            }
                            single_span =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            if v.is_null() {
                                continue;
                            }
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            start_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SpansAttributes {
                    attributes,
                    custom,
                    end_timestamp,
                    env,
                    host,
                    ingestion_reason,
                    parent_id,
                    resource_hash,
                    resource_name,
                    retained_by,
                    service,
                    single_span,
                    span_id,
                    start_timestamp,
                    tags,
                    trace_id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansAttributesVisitor)
    }
}
