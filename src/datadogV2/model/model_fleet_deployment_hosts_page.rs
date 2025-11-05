// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination details for the list of hosts in a deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentHostsPage {
    /// Current page index (zero-based).
    #[serde(rename = "current_page")]
    pub current_page: Option<i64>,
    /// Number of hosts returned per page.
    #[serde(rename = "page_size")]
    pub page_size: Option<i64>,
    /// Total number of hosts in this deployment.
    #[serde(rename = "total_hosts")]
    pub total_hosts: Option<i64>,
    /// Total number of pages available.
    #[serde(rename = "total_pages")]
    pub total_pages: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentHostsPage {
    pub fn new() -> FleetDeploymentHostsPage {
        FleetDeploymentHostsPage {
            current_page: None,
            page_size: None,
            total_hosts: None,
            total_pages: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_hosts(mut self, value: i64) -> Self {
        self.total_hosts = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
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

impl Default for FleetDeploymentHostsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentHostsPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentHostsPageVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentHostsPageVisitor {
            type Value = FleetDeploymentHostsPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_page: Option<i64> = None;
                let mut page_size: Option<i64> = None;
                let mut total_hosts: Option<i64> = None;
                let mut total_pages: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current_page" => {
                            if v.is_null() {
                                continue;
                            }
                            current_page =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_size" => {
                            if v.is_null() {
                                continue;
                            }
                            page_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            total_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_pages" => {
                            if v.is_null() {
                                continue;
                            }
                            total_pages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetDeploymentHostsPage {
                    current_page,
                    page_size,
                    total_hosts,
                    total_pages,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentHostsPageVisitor)
    }
}
