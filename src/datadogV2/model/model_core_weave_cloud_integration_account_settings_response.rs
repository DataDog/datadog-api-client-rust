// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Settings configured on the CoreWeave integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudIntegrationAccountSettingsResponse {
    /// Tags added to all data collected from this CoreWeave account.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// Tag-filter rules limiting metric collection to matching clusters. Absent when no filters are configured.
    #[serde(rename = "cluster_filters")]
    pub cluster_filters: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudIntegrationAccountSettingsResponse {
    pub fn new() -> CoreWeaveCloudIntegrationAccountSettingsResponse {
        CoreWeaveCloudIntegrationAccountSettingsResponse {
            account_tags: None,
            cluster_filters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }
    
    pub fn cluster_filters(mut self, value: Vec<String>) -> Self {
        self.cluster_filters = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for CoreWeaveCloudIntegrationAccountSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudIntegrationAccountSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudIntegrationAccountSettingsResponseVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudIntegrationAccountSettingsResponseVisitor {
            type Value = CoreWeaveCloudIntegrationAccountSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Vec<String>> = None;
                let mut cluster_filters: Option<Vec<String>> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            account_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cluster_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = CoreWeaveCloudIntegrationAccountSettingsResponse {
                    account_tags,
                    cluster_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudIntegrationAccountSettingsResponseVisitor)
    }
}
