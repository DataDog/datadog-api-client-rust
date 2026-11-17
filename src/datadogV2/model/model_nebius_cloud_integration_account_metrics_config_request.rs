// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for metric collection from this Nebius Cloud account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountMetricsConfigRequest {
    /// Tag-filter rules limiting metric collection to matching Kubernetes clusters, for cost savings. Include rules (`key:value`) are combined with OR, exclusion rules (`!key:value`) are then applied with AND, and `?` and `*` are wildcards; for example `cluster:prod-*` collects only matching clusters and `!cluster:dev` excludes a cluster. Defaults to no filters: every cluster is collected. Only cluster-scoped metrics are filtered; billing and object storage metrics are always collected.
    #[serde(rename = "cluster_filters")]
    pub cluster_filters: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountMetricsConfigRequest {
    pub fn new() -> NebiusCloudIntegrationAccountMetricsConfigRequest {
        NebiusCloudIntegrationAccountMetricsConfigRequest {
            cluster_filters: None,
            _unparsed: false,
        }
    }

    pub fn cluster_filters(mut self, value: Vec<String>) -> Self {
        self.cluster_filters = Some(value);
        self
    }
}

impl Default for NebiusCloudIntegrationAccountMetricsConfigRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountMetricsConfigRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountMetricsConfigRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountMetricsConfigRequestVisitor {
            type Value = NebiusCloudIntegrationAccountMetricsConfigRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cluster_filters: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cluster_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudIntegrationAccountMetricsConfigRequest {
                    cluster_filters,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountMetricsConfigRequestVisitor)
    }
}
