// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Partial Elastic Cloud monitoring interface settings for updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudSettingsUpdate {
    /// Enable to collect shard allocation metrics.
    #[serde(rename = "cat_allocation_stats_enabled")]
    pub cat_allocation_stats_enabled: Option<bool>,
    /// Enable to collect index-specific stats.
    #[serde(rename = "detailed_index_stats_enabled")]
    pub detailed_index_stats_enabled: Option<bool>,
    /// Enable to collect metrics about the indices in your cluster.
    #[serde(rename = "index_stats_enabled")]
    pub index_stats_enabled: Option<bool>,
    /// Enable to collect metrics about pending tasks.
    #[serde(rename = "pending_task_stats_enabled")]
    pub pending_task_stats_enabled: Option<bool>,
    /// Enable to collect all metrics even if primary shard metric collection times out.
    #[serde(rename = "pshard_graceful_to_enabled")]
    pub pshard_graceful_to_enabled: Option<bool>,
    /// Enable to collect metrics over primary shards.
    #[serde(rename = "pshard_stats_enabled")]
    pub pshard_stats_enabled: Option<bool>,
    /// Enable to collect snapshot lifecycle management metrics.
    #[serde(rename = "slm_stats_enabled")]
    pub slm_stats_enabled: Option<bool>,
    /// Custom tags for this deployment.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Deployment URL.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudSettingsUpdate {
    pub fn new() -> ElasticCloudSettingsUpdate {
        ElasticCloudSettingsUpdate {
            cat_allocation_stats_enabled: None,
            detailed_index_stats_enabled: None,
            index_stats_enabled: None,
            pending_task_stats_enabled: None,
            pshard_graceful_to_enabled: None,
            pshard_stats_enabled: None,
            slm_stats_enabled: None,
            tags: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cat_allocation_stats_enabled(mut self, value: bool) -> Self {
        self.cat_allocation_stats_enabled = Some(value);
        self
    }

    pub fn detailed_index_stats_enabled(mut self, value: bool) -> Self {
        self.detailed_index_stats_enabled = Some(value);
        self
    }

    pub fn index_stats_enabled(mut self, value: bool) -> Self {
        self.index_stats_enabled = Some(value);
        self
    }

    pub fn pending_task_stats_enabled(mut self, value: bool) -> Self {
        self.pending_task_stats_enabled = Some(value);
        self
    }

    pub fn pshard_graceful_to_enabled(mut self, value: bool) -> Self {
        self.pshard_graceful_to_enabled = Some(value);
        self
    }

    pub fn pshard_stats_enabled(mut self, value: bool) -> Self {
        self.pshard_stats_enabled = Some(value);
        self
    }

    pub fn slm_stats_enabled(mut self, value: bool) -> Self {
        self.slm_stats_enabled = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for ElasticCloudSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ElasticCloudSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for ElasticCloudSettingsUpdateVisitor {
            type Value = ElasticCloudSettingsUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cat_allocation_stats_enabled: Option<bool> = None;
                let mut detailed_index_stats_enabled: Option<bool> = None;
                let mut index_stats_enabled: Option<bool> = None;
                let mut pending_task_stats_enabled: Option<bool> = None;
                let mut pshard_graceful_to_enabled: Option<bool> = None;
                let mut pshard_stats_enabled: Option<bool> = None;
                let mut slm_stats_enabled: Option<bool> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cat_allocation_stats_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            cat_allocation_stats_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detailed_index_stats_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            detailed_index_stats_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index_stats_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            index_stats_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pending_task_stats_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            pending_task_stats_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pshard_graceful_to_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            pshard_graceful_to_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pshard_stats_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            pshard_stats_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slm_stats_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            slm_stats_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ElasticCloudSettingsUpdate {
                    cat_allocation_stats_enabled,
                    detailed_index_stats_enabled,
                    index_stats_enabled,
                    pending_task_stats_enabled,
                    pshard_graceful_to_enabled,
                    pshard_stats_enabled,
                    slm_stats_enabled,
                    tags,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ElasticCloudSettingsUpdateVisitor)
    }
}
