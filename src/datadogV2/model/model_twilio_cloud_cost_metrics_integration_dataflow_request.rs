// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Your Twilio cost data, so that Twilio spend can be broken down and attributed in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Cost data appears in Cloud Cost Management within 24 hours of enabling this dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioCloudCostMetricsIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `false`; set to `true` to start collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioCloudCostMetricsIntegrationDataflowRequest {
    pub fn new() -> TwilioCloudCostMetricsIntegrationDataflowRequest {
        TwilioCloudCostMetricsIntegrationDataflowRequest {
            enabled: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for TwilioCloudCostMetricsIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TwilioCloudCostMetricsIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioCloudCostMetricsIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for TwilioCloudCostMetricsIntegrationDataflowRequestVisitor {
            type Value = TwilioCloudCostMetricsIntegrationDataflowRequest;

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

                let content =
                    TwilioCloudCostMetricsIntegrationDataflowRequest { enabled, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioCloudCostMetricsIntegrationDataflowRequestVisitor)
    }
}
