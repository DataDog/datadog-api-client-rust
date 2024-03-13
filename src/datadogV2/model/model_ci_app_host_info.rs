// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Contains information of the host running the pipeline, stage, job, or step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppHostInfo {
    /// FQDN of the host.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// A list of labels used to select or identify the node.
    #[serde(rename = "labels")]
    pub labels: Option<Vec<String>>,
    /// Name for the host.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The path where the code is checked out.
    #[serde(rename = "workspace")]
    pub workspace: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppHostInfo {
    pub fn new() -> CIAppHostInfo {
        CIAppHostInfo {
            hostname: None,
            labels: None,
            name: None,
            workspace: None,
            _unparsed: false,
        }
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn workspace(mut self, value: String) -> Self {
        self.workspace = Some(value);
        self
    }
}

impl Default for CIAppHostInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppHostInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppHostInfoVisitor;
        impl<'a> Visitor<'a> for CIAppHostInfoVisitor {
            type Value = CIAppHostInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hostname: Option<String> = None;
                let mut labels: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut workspace: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "labels" => {
                            if v.is_null() {
                                continue;
                            }
                            labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace" => {
                            if v.is_null() {
                                continue;
                            }
                            workspace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CIAppHostInfo {
                    hostname,
                    labels,
                    name,
                    workspace,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppHostInfoVisitor)
    }
}
