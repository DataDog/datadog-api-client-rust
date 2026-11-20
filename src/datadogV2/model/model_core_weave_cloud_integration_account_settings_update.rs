// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Settings for updating the CoreWeave integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudIntegrationAccountSettingsUpdate {
    /// Tags added to all metrics and logs collected from this CoreWeave account. Sending it replaces the whole list; send an empty array to remove all tags, or omit it to keep the stored list.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// Tag-filter rules limiting metric collection to matching clusters. Sending it replaces the whole list; send an empty array to remove all filters and collect every cluster, or omit it to keep the stored list. Include rules (`key:value`) are combined with OR, exclusion rules (`!key:value`) are then applied with AND, and `?` and `*` are wildcards.
    #[serde(rename = "cluster_filters")]
    pub cluster_filters: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudIntegrationAccountSettingsUpdate {
    pub fn new() -> CoreWeaveCloudIntegrationAccountSettingsUpdate {
        CoreWeaveCloudIntegrationAccountSettingsUpdate {
            account_tags: None,
            cluster_filters: None,
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
    
}


impl Default for CoreWeaveCloudIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudIntegrationAccountSettingsUpdateVisitor {
            type Value = CoreWeaveCloudIntegrationAccountSettingsUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Vec<String>> = None;
                let mut cluster_filters: Option<Vec<String>> = None;
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
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = CoreWeaveCloudIntegrationAccountSettingsUpdate {
                    account_tags,
                    cluster_filters,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudIntegrationAccountSettingsUpdateVisitor)
    }
}
