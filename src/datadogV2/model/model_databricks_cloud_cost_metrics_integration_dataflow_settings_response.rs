// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Databricks cloud cost metrics dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
    /// Whether cost data is collected for every workspace in the Databricks account rather than this workspace only.
    #[serde(rename = "ccm_collect_all_workspaces")]
    pub ccm_collect_all_workspaces: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
    pub fn new() -> DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
        DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
            ccm_collect_all_workspaces: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ccm_collect_all_workspaces(mut self, value: bool) -> Self {
        self.ccm_collect_all_workspaces = Some(value);
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

impl Default for DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksCloudCostMetricsIntegrationDataflowSettingsResponseVisitor;
        impl<'a> Visitor<'a> for DatabricksCloudCostMetricsIntegrationDataflowSettingsResponseVisitor {
            type Value = DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ccm_collect_all_workspaces: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DatabricksCloudCostMetricsIntegrationDataflowSettingsResponse {
                    ccm_collect_all_workspaces,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(DatabricksCloudCostMetricsIntegrationDataflowSettingsResponseVisitor)
    }
}
