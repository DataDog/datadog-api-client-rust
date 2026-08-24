// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dataflows to configure on the Elastic Cloud integration account, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudIntegrationDataflowsRequest {
    /// The Elastic Cloud detailed index stats dataflow.
    #[serde(rename = "elastic-cloud-detailed-index-stats")]
    pub elastic_cloud_detailed_index_stats:
        Option<crate::datadogV2::model::ElasticCloudDetailedIndexStatsIntegrationDataflowRequest>,
    /// The Elastic Cloud index stats dataflow.
    #[serde(rename = "elastic-cloud-index-stats")]
    pub elastic_cloud_index_stats:
        Option<crate::datadogV2::model::ElasticCloudIndexStatsIntegrationDataflowRequest>,
    /// The Elastic Cloud pending task stats dataflow.
    #[serde(rename = "elastic-cloud-pending-task-stats")]
    pub elastic_cloud_pending_task_stats:
        Option<crate::datadogV2::model::ElasticCloudPendingTaskStatsIntegrationDataflowRequest>,
    /// The Elastic Cloud primary shard graceful timeout dataflow.
    #[serde(rename = "elastic-cloud-primary-shard-graceful-timeout")]
    pub elastic_cloud_primary_shard_graceful_timeout: Option<
        crate::datadogV2::model::ElasticCloudPrimaryShardGracefulTimeoutIntegrationDataflowRequest,
    >,
    /// The Elastic Cloud primary shard stats dataflow.
    #[serde(rename = "elastic-cloud-primary-shard-stats")]
    pub elastic_cloud_primary_shard_stats:
        Option<crate::datadogV2::model::ElasticCloudPrimaryShardStatsIntegrationDataflowRequest>,
    /// The Elastic Cloud shard allocation stats dataflow.
    #[serde(rename = "elastic-cloud-shard-allocation-stats")]
    pub elastic_cloud_shard_allocation_stats:
        Option<crate::datadogV2::model::ElasticCloudShardAllocationStatsIntegrationDataflowRequest>,
    /// The Elastic Cloud snapshot lifecycle management stats dataflow.
    #[serde(rename = "elastic-cloud-slm-stats")]
    pub elastic_cloud_slm_stats:
        Option<crate::datadogV2::model::ElasticCloudSlmStatsIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudIntegrationDataflowsRequest {
    pub fn new() -> ElasticCloudIntegrationDataflowsRequest {
        ElasticCloudIntegrationDataflowsRequest {
            elastic_cloud_detailed_index_stats: None,
            elastic_cloud_index_stats: None,
            elastic_cloud_pending_task_stats: None,
            elastic_cloud_primary_shard_graceful_timeout: None,
            elastic_cloud_primary_shard_stats: None,
            elastic_cloud_shard_allocation_stats: None,
            elastic_cloud_slm_stats: None,
            _unparsed: false,
        }
    }

    pub fn elastic_cloud_detailed_index_stats(
        mut self,
        value: crate::datadogV2::model::ElasticCloudDetailedIndexStatsIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_detailed_index_stats = Some(value);
        self
    }

    pub fn elastic_cloud_index_stats(
        mut self,
        value: crate::datadogV2::model::ElasticCloudIndexStatsIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_index_stats = Some(value);
        self
    }

    pub fn elastic_cloud_pending_task_stats(
        mut self,
        value: crate::datadogV2::model::ElasticCloudPendingTaskStatsIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_pending_task_stats = Some(value);
        self
    }

    pub fn elastic_cloud_primary_shard_graceful_timeout(
        mut self,
        value: crate::datadogV2::model::ElasticCloudPrimaryShardGracefulTimeoutIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_primary_shard_graceful_timeout = Some(value);
        self
    }

    pub fn elastic_cloud_primary_shard_stats(
        mut self,
        value: crate::datadogV2::model::ElasticCloudPrimaryShardStatsIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_primary_shard_stats = Some(value);
        self
    }

    pub fn elastic_cloud_shard_allocation_stats(
        mut self,
        value: crate::datadogV2::model::ElasticCloudShardAllocationStatsIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_shard_allocation_stats = Some(value);
        self
    }

    pub fn elastic_cloud_slm_stats(
        mut self,
        value: crate::datadogV2::model::ElasticCloudSlmStatsIntegrationDataflowRequest,
    ) -> Self {
        self.elastic_cloud_slm_stats = Some(value);
        self
    }
}

impl Default for ElasticCloudIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ElasticCloudIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for ElasticCloudIntegrationDataflowsRequestVisitor {
            type Value = ElasticCloudIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elastic_cloud_detailed_index_stats: Option<crate::datadogV2::model::ElasticCloudDetailedIndexStatsIntegrationDataflowRequest> = None;
                let mut elastic_cloud_index_stats: Option<
                    crate::datadogV2::model::ElasticCloudIndexStatsIntegrationDataflowRequest,
                > = None;
                let mut elastic_cloud_pending_task_stats: Option<
                    crate::datadogV2::model::ElasticCloudPendingTaskStatsIntegrationDataflowRequest,
                > = None;
                let mut elastic_cloud_primary_shard_graceful_timeout: Option<crate::datadogV2::model::ElasticCloudPrimaryShardGracefulTimeoutIntegrationDataflowRequest> = None;
                let mut elastic_cloud_primary_shard_stats: Option<crate::datadogV2::model::ElasticCloudPrimaryShardStatsIntegrationDataflowRequest> = None;
                let mut elastic_cloud_shard_allocation_stats: Option<crate::datadogV2::model::ElasticCloudShardAllocationStatsIntegrationDataflowRequest> = None;
                let mut elastic_cloud_slm_stats: Option<
                    crate::datadogV2::model::ElasticCloudSlmStatsIntegrationDataflowRequest,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elastic-cloud-detailed-index-stats" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_detailed_index_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elastic-cloud-index-stats" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_index_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elastic-cloud-pending-task-stats" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_pending_task_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elastic-cloud-primary-shard-graceful-timeout" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_primary_shard_graceful_timeout =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elastic-cloud-primary-shard-stats" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_primary_shard_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elastic-cloud-shard-allocation-stats" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_shard_allocation_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elastic-cloud-slm-stats" => {
                            if v.is_null() {
                                continue;
                            }
                            elastic_cloud_slm_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = ElasticCloudIntegrationDataflowsRequest {
                    elastic_cloud_detailed_index_stats,
                    elastic_cloud_index_stats,
                    elastic_cloud_pending_task_stats,
                    elastic_cloud_primary_shard_graceful_timeout,
                    elastic_cloud_primary_shard_stats,
                    elastic_cloud_shard_allocation_stats,
                    elastic_cloud_slm_stats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ElasticCloudIntegrationDataflowsRequestVisitor)
    }
}
