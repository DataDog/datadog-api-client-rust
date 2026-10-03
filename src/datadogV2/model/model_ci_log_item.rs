// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A CI job log line.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CILogItem {
    /// Comma-separated tags in `key:value` format. A log can have up to 256 tags, including repeated keys.
    #[serde(rename = "ddtags")]
    pub ddtags: Option<String>,
    /// The job event's `resource.id`, sent through the CI Visibility pipeline API.
    #[serde(rename = "job_id")]
    pub job_id: String,
    /// The line number in the job log. Use 0 or 1 for the first line.
    #[serde(rename = "line_number")]
    pub line_number: Option<i64>,
    /// The non-empty log line message.
    #[serde(rename = "message")]
    pub message: String,
    /// The `resource.unique_id` of the pipeline event, which must also match the job event's
    /// `resource.pipeline_unique_id`.
    #[serde(rename = "pipeline_unique_id")]
    pub pipeline_unique_id: String,
    /// The provider name sent with the pipeline event. It defaults to `custom` when omitted and, when provided,
    /// must be non-empty and cannot contain a comma.
    #[serde(rename = "provider_name")]
    pub provider_name: Option<String>,
    /// The provider-defined section containing this log line, used to display collapsible groups of lines in the CI
    /// job log view.
    #[serde(rename = "section_name")]
    pub section_name: Option<String>,
    /// The status of this log line. Any string is accepted. Datadog maps non-empty values to a standard log status.
    /// See [status mapping](<https://docs.datadoghq.com/logs/log_configuration/processors/log_status_remapper/>).
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The log line time in RFC 3339 format with an explicit timezone. If omitted, the intake time is used. It can
    /// be at most 18 hours in the past or 12 hours in the future.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties:
        std::collections::BTreeMap<String, crate::datadogV2::model::CILogAttributeValue>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CILogItem {
    pub fn new(job_id: String, message: String, pipeline_unique_id: String) -> CILogItem {
        CILogItem {
            ddtags: None,
            job_id,
            line_number: None,
            message,
            pipeline_unique_id,
            provider_name: None,
            section_name: None,
            status: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ddtags(mut self, value: String) -> Self {
        self.ddtags = Some(value);
        self
    }

    pub fn line_number(mut self, value: i64) -> Self {
        self.line_number = Some(value);
        self
    }

    pub fn provider_name(mut self, value: String) -> Self {
        self.provider_name = Some(value);
        self
    }

    pub fn section_name(mut self, value: String) -> Self {
        self.section_name = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::CILogAttributeValue>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CILogItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CILogItemVisitor;
        impl<'a> Visitor<'a> for CILogItemVisitor {
            type Value = CILogItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ddtags: Option<String> = None;
                let mut job_id: Option<String> = None;
                let mut line_number: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut pipeline_unique_id: Option<String> = None;
                let mut provider_name: Option<String> = None;
                let mut section_name: Option<String> = None;
                let mut status: Option<String> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    crate::datadogV2::model::CILogAttributeValue,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ddtags" => {
                            if v.is_null() {
                                continue;
                            }
                            ddtags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "job_id" => {
                            job_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "line_number" => {
                            if v.is_null() {
                                continue;
                            }
                            line_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_unique_id" => {
                            pipeline_unique_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_name" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "section_name" => {
                            if v.is_null() {
                                continue;
                            }
                            section_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let job_id = job_id.ok_or_else(|| M::Error::missing_field("job_id"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let pipeline_unique_id = pipeline_unique_id
                    .ok_or_else(|| M::Error::missing_field("pipeline_unique_id"))?;

                let content = CILogItem {
                    ddtags,
                    job_id,
                    line_number,
                    message,
                    pipeline_unique_id,
                    provider_name,
                    section_name,
                    status,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CILogItemVisitor)
    }
}
