// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Logs that are sent over HTTP.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    pub fn ddsource(&mut self, value: String) -> &mut Self {
        self.ddsource = Some(value);
        self
    }

    pub fn ddtags(&mut self, value: String) -> &mut Self {
        self.ddtags = Some(value);
        self
    }

    pub fn hostname(&mut self, value: String) -> &mut Self {
        self.hostname = Some(value);
        self
    }

    pub fn service(&mut self, value: String) -> &mut Self {
        self.service = Some(value);
        self
    }
}
