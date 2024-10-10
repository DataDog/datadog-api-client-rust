// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An Opsgenie integration schema.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3DatadogIntegrationOpsgenie {
    /// The region for the Opsgenie integration.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The service URL for the Opsgenie integration.
    #[serde(rename = "serviceURL")]
    pub service_url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3DatadogIntegrationOpsgenie {
    pub fn new(service_url: String) -> EntityV3DatadogIntegrationOpsgenie {
        EntityV3DatadogIntegrationOpsgenie {
            region: None,
            service_url,
            _unparsed: false,
        }
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EntityV3DatadogIntegrationOpsgenie {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3DatadogIntegrationOpsgenieVisitor;
        impl<'a> Visitor<'a> for EntityV3DatadogIntegrationOpsgenieVisitor {
            type Value = EntityV3DatadogIntegrationOpsgenie;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut region: Option<String> = None;
                let mut service_url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serviceURL" => {
                            service_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let service_url =
                    service_url.ok_or_else(|| M::Error::missing_field("service_url"))?;

                let content = EntityV3DatadogIntegrationOpsgenie {
                    region,
                    service_url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3DatadogIntegrationOpsgenieVisitor)
    }
}
