// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from OpenAI Usage and Costs, keyed by dataflow ID. Datadog always collects organization usage metrics. Each dataflow in this object adds another type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIUsageAndCostsIntegrationDataflowsRequest {
    /// OpenAI organization costs, which you can analyze in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). See [collected data](<https://docs.datadoghq.com/cloud_cost_management/saas_costs/?tab=openai#data-collected>) for what is included. Cost data appears in Cloud Cost Management within 24 hours after you enable this dataflow.
    #[serde(rename = "openai-cloud-cost-data")]
    pub openai_cloud_cost_data:
        Option<crate::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAIUsageAndCostsIntegrationDataflowsRequest {
    pub fn new() -> OpenAIUsageAndCostsIntegrationDataflowsRequest {
        OpenAIUsageAndCostsIntegrationDataflowsRequest {
            openai_cloud_cost_data: None,
            _unparsed: false,
        }
    }

    pub fn openai_cloud_cost_data(
        mut self,
        value: crate::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest,
    ) -> Self {
        self.openai_cloud_cost_data = Some(value);
        self
    }
}

impl Default for OpenAIUsageAndCostsIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAIUsageAndCostsIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIUsageAndCostsIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for OpenAIUsageAndCostsIntegrationDataflowsRequestVisitor {
            type Value = OpenAIUsageAndCostsIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_cloud_cost_data: Option<crate::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "openai-cloud-cost-data" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_cloud_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OpenAIUsageAndCostsIntegrationDataflowsRequest {
                    openai_cloud_cost_data,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAIUsageAndCostsIntegrationDataflowsRequestVisitor)
    }
}
