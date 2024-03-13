// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object representing a host.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Host {
    /// Host aliases collected by Datadog.
    #[serde(rename = "aliases")]
    pub aliases: Option<Vec<String>>,
    /// The Datadog integrations reporting metrics for the host.
    #[serde(rename = "apps")]
    pub apps: Option<Vec<String>>,
    /// AWS name of your host.
    #[serde(rename = "aws_name")]
    pub aws_name: Option<String>,
    /// The host name.
    #[serde(rename = "host_name")]
    pub host_name: Option<String>,
    /// The host ID.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// If a host is muted or unmuted.
    #[serde(rename = "is_muted")]
    pub is_muted: Option<bool>,
    /// Last time the host reported a metric data point.
    #[serde(rename = "last_reported_time")]
    pub last_reported_time: Option<i64>,
    /// Metadata associated with your host.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::HostMeta>,
    /// Host Metrics collected.
    #[serde(rename = "metrics")]
    pub metrics: Option<crate::datadogV1::model::HostMetrics>,
    /// Timeout of the mute applied to your host.
    #[serde(
        rename = "mute_timeout",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub mute_timeout: Option<Option<i64>>,
    /// The host name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Source or cloud provider associated with your host.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,
    /// List of tags for each source (AWS, Datadog Agent, Chef..).
    #[serde(rename = "tags_by_source")]
    pub tags_by_source: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// Displays UP when the expected metrics are received and displays `???` if no metrics are received.
    #[serde(rename = "up")]
    pub up: Option<bool>,
}

impl Host {
    pub fn new() -> Host {
        Host {
            aliases: None,
            apps: None,
            aws_name: None,
            host_name: None,
            id: None,
            is_muted: None,
            last_reported_time: None,
            meta: None,
            metrics: None,
            mute_timeout: None,
            name: None,
            sources: None,
            tags_by_source: None,
            up: None,
        }
    }

    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn apps(mut self, value: Vec<String>) -> Self {
        self.apps = Some(value);
        self
    }

    pub fn aws_name(mut self, value: String) -> Self {
        self.aws_name = Some(value);
        self
    }

    pub fn host_name(mut self, value: String) -> Self {
        self.host_name = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_muted(mut self, value: bool) -> Self {
        self.is_muted = Some(value);
        self
    }

    pub fn last_reported_time(mut self, value: i64) -> Self {
        self.last_reported_time = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV1::model::HostMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn metrics(mut self, value: crate::datadogV1::model::HostMetrics) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn mute_timeout(mut self, value: Option<i64>) -> Self {
        self.mute_timeout = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn tags_by_source(
        mut self,
        value: std::collections::BTreeMap<String, Vec<String>>,
    ) -> Self {
        self.tags_by_source = Some(value);
        self
    }

    pub fn up(mut self, value: bool) -> Self {
        self.up = Some(value);
        self
    }
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}
