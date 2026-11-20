// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Settings configured on the Nebius Cloud integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountSettingsResponse {
    /// Tags applied to all data collected from this Nebius Cloud account.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// Metric collection settings configured on the Nebius Cloud integration account. Absent or empty when no filters are configured.
    #[serde(rename = "metrics_config")]
    pub metrics_config: Option<crate::datadogV2::model::NebiusCloudIntegrationAccountMetricsConfigResponse>,
    /// ID of the Nebius tenant being monitored.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl NebiusCloudIntegrationAccountSettingsResponse {
    pub fn new(tenant_id: String) -> NebiusCloudIntegrationAccountSettingsResponse {
        NebiusCloudIntegrationAccountSettingsResponse {
            account_tags: None,
            metrics_config: None,
            tenant_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }
    
    pub fn metrics_config(mut self, value: crate::datadogV2::model::NebiusCloudIntegrationAccountMetricsConfigResponse) -> Self {
        self.metrics_config = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountSettingsResponseVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountSettingsResponseVisitor {
            type Value = NebiusCloudIntegrationAccountSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Vec<String>> = None;
                let mut metrics_config: Option<crate::datadogV2::model::NebiusCloudIntegrationAccountMetricsConfigResponse> = None;
                let mut tenant_id: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            account_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metrics_config" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics_config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;

                let content = NebiusCloudIntegrationAccountSettingsResponse {
                    account_tags,
                    metrics_config,
                    tenant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountSettingsResponseVisitor)
    }
}
