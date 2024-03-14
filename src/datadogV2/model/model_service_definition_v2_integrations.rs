// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Third party integrations that Datadog supports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Integrations {
    /// Opsgenie integration for the service.
    #[serde(rename = "opsgenie")]
    pub opsgenie: Option<crate::datadogV2::model::ServiceDefinitionV2Opsgenie>,
    /// PagerDuty service URL for the service.
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Integrations {
    pub fn new() -> ServiceDefinitionV2Integrations {
        ServiceDefinitionV2Integrations {
            opsgenie: None,
            pagerduty: None,
            _unparsed: false,
        }
    }

    pub fn opsgenie(mut self, value: crate::datadogV2::model::ServiceDefinitionV2Opsgenie) -> Self {
        self.opsgenie = Some(value);
        self
    }

    pub fn pagerduty(mut self, value: String) -> Self {
        self.pagerduty = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV2Integrations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Integrations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2IntegrationsVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2IntegrationsVisitor {
            type Value = ServiceDefinitionV2Integrations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut opsgenie: Option<crate::datadogV2::model::ServiceDefinitionV2Opsgenie> =
                    None;
                let mut pagerduty: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "opsgenie" => {
                            if v.is_null() {
                                continue;
                            }
                            opsgenie = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pagerduty" => {
                            if v.is_null() {
                                continue;
                            }
                            pagerduty = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionV2Integrations {
                    opsgenie,
                    pagerduty,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2IntegrationsVisitor)
    }
}
