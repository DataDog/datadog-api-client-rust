// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Opsgenie integration for the service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Dot2Opsgenie {
    /// Opsgenie instance region.
    #[serde(rename = "region")]
    pub region: Option<crate::datadogV2::model::ServiceDefinitionV2Dot2OpsgenieRegion>,
    /// Opsgenie service url.
    #[serde(rename = "service-url")]
    pub service_url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Dot2Opsgenie {
    pub fn new(service_url: String) -> ServiceDefinitionV2Dot2Opsgenie {
        ServiceDefinitionV2Dot2Opsgenie {
            region: None,
            service_url,
            _unparsed: false,
        }
    }

    pub fn region(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot2OpsgenieRegion,
    ) -> Self {
        self.region = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot2Opsgenie {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2Dot2OpsgenieVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2Dot2OpsgenieVisitor {
            type Value = ServiceDefinitionV2Dot2Opsgenie;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut region: Option<
                    crate::datadogV2::model::ServiceDefinitionV2Dot2OpsgenieRegion,
                > = None;
                let mut service_url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _region) = region {
                                match _region {
                                    crate::datadogV2::model::ServiceDefinitionV2Dot2OpsgenieRegion::UnparsedObject(_region) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "service-url" => {
                            service_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let service_url =
                    service_url.ok_or_else(|| M::Error::missing_field("service_url"))?;

                let content = ServiceDefinitionV2Dot2Opsgenie {
                    region,
                    service_url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2Dot2OpsgenieVisitor)
    }
}
