// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Databricks data observability dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
    /// Cron expression setting how often the data observability crawlers run.
    #[serde(rename = "do_crawlers_cron")]
    pub do_crawlers_cron: Option<String>,
    /// Whether the Databricks `system` catalog is synchronized alongside your data catalogs.
    #[serde(rename = "sync_system_catalog")]
    pub sync_system_catalog: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
    pub fn new() -> DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
        DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
            do_crawlers_cron: None,
            sync_system_catalog: None,
            _unparsed: false,
        }
    }

    pub fn do_crawlers_cron(mut self, value: String) -> Self {
        self.do_crawlers_cron = Some(value);
        self
    }

    pub fn sync_system_catalog(mut self, value: bool) -> Self {
        self.sync_system_catalog = Some(value);
        self
    }
}

impl Default for DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksDataObservabilityIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for DatabricksDataObservabilityIntegrationDataflowSettingsRequestVisitor {
            type Value = DatabricksDataObservabilityIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut do_crawlers_cron: Option<String> = None;
                let mut sync_system_catalog: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "do_crawlers_cron" => {
                            if v.is_null() {
                                continue;
                            }
                            do_crawlers_cron =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_system_catalog" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_system_catalog =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = DatabricksDataObservabilityIntegrationDataflowSettingsRequest {
                    do_crawlers_cron,
                    sync_system_catalog,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(DatabricksDataObservabilityIntegrationDataflowSettingsRequestVisitor)
    }
}
