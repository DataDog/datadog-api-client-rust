// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// PagerDuty integration for the service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Dot1Pagerduty {
    /// PagerDuty service url.
    #[serde(rename = "service-url")]
    pub service_url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Dot1Pagerduty {
    pub fn new() -> ServiceDefinitionV2Dot1Pagerduty {
        ServiceDefinitionV2Dot1Pagerduty {
            service_url: None,
            _unparsed: false,
        }
    }

    pub fn service_url(&mut self, value: String) -> &mut Self {
        self.service_url = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV2Dot1Pagerduty {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot1Pagerduty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2Dot1PagerdutyVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2Dot1PagerdutyVisitor {
            type Value = ServiceDefinitionV2Dot1Pagerduty;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut service_url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "service-url" => {
                            if v.is_null() {
                                continue;
                            }
                            service_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionV2Dot1Pagerduty {
                    service_url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2Dot1PagerdutyVisitor)
    }
}
