// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dictionary containing the key `excluded_resource_providers` which has to be a list of Microsoft Azure Resource Provider names.
/// This feature is currently being beta tested.
/// In order to enable all resource providers for metric collection, pass:
/// `metrics_config: {"excluded_resource_providers": []}` (i.e., an empty list for `excluded_resource_providers`).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AzureAccountMetricsConfig {
    /// List of Microsoft Azure Resource Providers to exclude from metric collection.
    #[serde(rename = "excluded_resource_providers")]
    pub excluded_resource_providers: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AzureAccountMetricsConfig {
    pub fn new() -> AzureAccountMetricsConfig {
        AzureAccountMetricsConfig {
            excluded_resource_providers: None,
            _unparsed: false,
        }
    }

    pub fn excluded_resource_providers(mut self, value: Vec<String>) -> Self {
        self.excluded_resource_providers = Some(value);
        self
    }
}

impl Default for AzureAccountMetricsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AzureAccountMetricsConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureAccountMetricsConfigVisitor;
        impl<'a> Visitor<'a> for AzureAccountMetricsConfigVisitor {
            type Value = AzureAccountMetricsConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut excluded_resource_providers: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "excluded_resource_providers" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_resource_providers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AzureAccountMetricsConfig {
                    excluded_resource_providers,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureAccountMetricsConfigVisitor)
    }
}
