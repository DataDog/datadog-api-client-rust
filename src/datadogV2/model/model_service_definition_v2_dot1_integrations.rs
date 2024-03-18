// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Third party integrations that Datadog supports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Dot1Integrations {
    /// Opsgenie integration for the service.
    #[serde(rename = "opsgenie")]
    pub opsgenie: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Opsgenie>,
    /// PagerDuty integration for the service.
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Pagerduty>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Dot1Integrations {
    pub fn new() -> ServiceDefinitionV2Dot1Integrations {
        ServiceDefinitionV2Dot1Integrations {
            opsgenie: None,
            pagerduty: None,
            _unparsed: false,
        }
    }

    pub fn opsgenie(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1Opsgenie,
    ) -> Self {
        self.opsgenie = Some(value);
        self
    }

    pub fn pagerduty(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1Pagerduty,
    ) -> Self {
        self.pagerduty = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV2Dot1Integrations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot1Integrations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2Dot1IntegrationsVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2Dot1IntegrationsVisitor {
            type Value = ServiceDefinitionV2Dot1Integrations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut opsgenie: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Opsgenie> =
                    None;
                let mut pagerduty: Option<
                    crate::datadogV2::model::ServiceDefinitionV2Dot1Pagerduty,
                > = None;
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

                let content = ServiceDefinitionV2Dot1Integrations {
                    opsgenie,
                    pagerduty,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2Dot1IntegrationsVisitor)
    }
}
