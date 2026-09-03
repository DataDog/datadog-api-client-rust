// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Databricks cloud cost metrics dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
    /// Whether cost data is collected for every workspace in the Databricks account rather than this workspace only.
    #[serde(rename = "ccm_collect_all_workspaces")]
    pub ccm_collect_all_workspaces: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
    pub fn new() -> DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
        DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
            ccm_collect_all_workspaces: None,
            _unparsed: false,
        }
    }

    pub fn ccm_collect_all_workspaces(mut self, value: bool) -> Self {
        self.ccm_collect_all_workspaces = Some(value);
        self
    }
}

impl Default for DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksCloudCostMetricsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for DatabricksCloudCostMetricsIntegrationDataflowSettingsRequestVisitor {
            type Value = DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ccm_collect_all_workspaces: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ccm_collect_all_workspaces" => {
                            if v.is_null() {
                                continue;
                            }
                            ccm_collect_all_workspaces =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest {
                    ccm_collect_all_workspaces,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(DatabricksCloudCostMetricsIntegrationDataflowSettingsRequestVisitor)
    }
}
