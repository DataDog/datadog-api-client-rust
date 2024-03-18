// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Basic information about a service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV1Info {
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// A short description of the service.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A friendly name of the service.
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    /// Service tier.
    #[serde(rename = "service-tier")]
    pub service_tier: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV1Info {
    pub fn new(dd_service: String) -> ServiceDefinitionV1Info {
        ServiceDefinitionV1Info {
            dd_service,
            description: None,
            display_name: None,
            service_tier: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn service_tier(mut self, value: String) -> Self {
        self.service_tier = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV1Info {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV1InfoVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV1InfoVisitor {
            type Value = ServiceDefinitionV1Info;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dd_service: Option<String> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut service_tier: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dd-service" => {
                            dd_service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display-name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service-tier" => {
                            if v.is_null() {
                                continue;
                            }
                            service_tier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let dd_service = dd_service.ok_or_else(|| M::Error::missing_field("dd_service"))?;

                let content = ServiceDefinitionV1Info {
                    dd_service,
                    description,
                    display_name,
                    service_tier,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV1InfoVisitor)
    }
}
