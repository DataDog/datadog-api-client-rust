// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Costs of the OpenAI project that owns the key in `authentication`. Analyze these costs in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Cost data appears in Cloud Cost Management within 24 hours after you enable this dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `false`. Set to `true` to start collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
    pub fn new() -> OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
        OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
            enabled: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequestVisitor {
            type Value = OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest {
                    enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequestVisitor,
        )
    }
}
