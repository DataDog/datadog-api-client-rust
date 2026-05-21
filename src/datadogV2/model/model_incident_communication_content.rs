// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The content of a communication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCommunicationContent {
    /// A key used for grouping communications.
    #[serde(rename = "grouping_key")]
    pub grouping_key: Option<String>,
    /// The list of handles the communication is sent to.
    #[serde(rename = "handles")]
    pub handles: Vec<crate::datadogV2::model::IncidentCommunicationContentHandle>,
    /// The message body of the communication.
    #[serde(rename = "message")]
    pub message: String,
    /// The status code of the communication.
    #[serde(rename = "status")]
    pub status: Option<i32>,
    /// The subject line of the communication.
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCommunicationContent {
    pub fn new(
        handles: Vec<crate::datadogV2::model::IncidentCommunicationContentHandle>,
        message: String,
    ) -> IncidentCommunicationContent {
        IncidentCommunicationContent {
            grouping_key: None,
            handles,
            message,
            status: None,
            subject: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn grouping_key(mut self, value: String) -> Self {
        self.grouping_key = Some(value);
        self
    }

    pub fn status(mut self, value: i32) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subject(mut self, value: String) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentCommunicationContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCommunicationContentVisitor;
        impl<'a> Visitor<'a> for IncidentCommunicationContentVisitor {
            type Value = IncidentCommunicationContent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut grouping_key: Option<String> = None;
                let mut handles: Option<
                    Vec<crate::datadogV2::model::IncidentCommunicationContentHandle>,
                > = None;
                let mut message: Option<String> = None;
                let mut status: Option<i32> = None;
                let mut subject: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "grouping_key" => {
                            if v.is_null() {
                                continue;
                            }
                            grouping_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handles" => {
                            handles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            if v.is_null() {
                                continue;
                            }
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handles = handles.ok_or_else(|| M::Error::missing_field("handles"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;

                let content = IncidentCommunicationContent {
                    grouping_key,
                    handles,
                    message,
                    status,
                    subject,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCommunicationContentVisitor)
    }
}
