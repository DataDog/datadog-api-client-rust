// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing a host.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Host {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostVisitor;
        impl<'a> Visitor<'a> for HostVisitor {
            type Value = Host;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aliases: Option<Vec<String>> = None;
                let mut apps: Option<Vec<String>> = None;
                let mut aws_name: Option<String> = None;
                let mut host_name: Option<String> = None;
                let mut id: Option<i64> = None;
                let mut is_muted: Option<bool> = None;
                let mut last_reported_time: Option<i64> = None;
                let mut meta: Option<crate::datadogV1::model::HostMeta> = None;
                let mut metrics: Option<crate::datadogV1::model::HostMetrics> = None;
                let mut mute_timeout: Option<Option<i64>> = None;
                let mut name: Option<String> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut tags_by_source: Option<std::collections::BTreeMap<String, Vec<String>>> =
                    None;
                let mut up: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aliases" => {
                            if v.is_null() {
                                continue;
                            }
                            aliases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apps" => {
                            if v.is_null() {
                                continue;
                            }
                            apps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_name" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_name" => {
                            if v.is_null() {
                                continue;
                            }
                            host_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_muted" => {
                            if v.is_null() {
                                continue;
                            }
                            is_muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_reported_time" => {
                            if v.is_null() {
                                continue;
                            }
                            last_reported_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mute_timeout" => {
                            mute_timeout =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags_by_source" => {
                            if v.is_null() {
                                continue;
                            }
                            tags_by_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "up" => {
                            if v.is_null() {
                                continue;
                            }
                            up = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = Host {
                    aliases,
                    apps,
                    aws_name,
                    host_name,
                    id,
                    is_muted,
                    last_reported_time,
                    meta,
                    metrics,
                    mute_timeout,
                    name,
                    sources,
                    tags_by_source,
                    up,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostVisitor)
    }
}
