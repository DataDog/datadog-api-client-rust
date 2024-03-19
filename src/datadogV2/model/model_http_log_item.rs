// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Logs that are sent over HTTP.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HTTPLogItem {
    /// The integration name associated with your log: the technology from which the log originated.
    /// When it matches an integration name, Datadog automatically installs the corresponding parsers and facets.
    /// See [reserved attributes](<https://docs.datadoghq.com/logs/log_configuration/attributes_naming_convention/#reserved-attributes>).
    #[serde(rename = "ddsource")]
    pub ddsource: Option<String>,
    /// Tags associated with your logs.
    #[serde(rename = "ddtags")]
    pub ddtags: Option<String>,
    /// The name of the originating host of the log.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// The message [reserved attribute](<https://docs.datadoghq.com/logs/log_configuration/attributes_naming_convention/#reserved-attributes>)
    /// of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry.
    /// That value is then highlighted and displayed in the Logstream, where it is indexed for full text search.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the application or service generating the log events.
    /// It is used to switch from Logs to APM, so make sure you define the same value when you use both products.
    /// See [reserved attributes](<https://docs.datadoghq.com/logs/log_configuration/attributes_naming_convention/#reserved-attributes>).
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HTTPLogItem {
    pub fn new(message: String) -> HTTPLogItem {
        HTTPLogItem {
            ddsource: None,
            ddtags: None,
            hostname: None,
            message,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ddsource(mut self, value: String) -> Self {
        self.ddsource = Some(value);
        self
    }

    pub fn ddtags(mut self, value: String) -> Self {
        self.ddtags = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, String>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for HTTPLogItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HTTPLogItemVisitor;
        impl<'a> Visitor<'a> for HTTPLogItemVisitor {
            type Value = HTTPLogItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ddsource: Option<String> = None;
                let mut ddtags: Option<String> = None;
                let mut hostname: Option<String> = None;
                let mut message: Option<String> = None;
                let mut service: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<String, String> =
                    std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ddsource" => {
                            if v.is_null() {
                                continue;
                            }
                            ddsource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ddtags" => {
                            if v.is_null() {
                                continue;
                            }
                            ddtags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;

                let content = HTTPLogItem {
                    ddsource,
                    ddtags,
                    hostname,
                    message,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HTTPLogItemVisitor)
    }
}
