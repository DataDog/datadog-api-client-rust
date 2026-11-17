// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cloud cost data read from a Nebius Object Storage bucket export and used by [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Enabling it requires a bucket holding your cost export and a Nebius access key pair that can read it; a request that enables this dataflow without them is rejected with a `422` response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudCostCollectionIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `false`; set to `true` to start collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Settings of the Cloud Cost Management dataflow. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings: Option<
        crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowSettingsRequest,
    >,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudCostCollectionIntegrationDataflowRequest {
    pub fn new() -> NebiusCloudCostCollectionIntegrationDataflowRequest {
        NebiusCloudCostCollectionIntegrationDataflowRequest {
            enabled: None,
            settings: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowSettingsRequest,
    ) -> Self {
        self.settings = Some(value);
        self
    }
}

impl Default for NebiusCloudCostCollectionIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudCostCollectionIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudCostCollectionIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudCostCollectionIntegrationDataflowRequestVisitor {
            type Value = NebiusCloudCostCollectionIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut settings: Option<crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowSettingsRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudCostCollectionIntegrationDataflowRequest {
                    enabled,
                    settings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudCostCollectionIntegrationDataflowRequestVisitor)
    }
}
