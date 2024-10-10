// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A base schema for defining third-party integrations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3Integrations {
    /// An Opsgenie integration schema.
    #[serde(rename = "opsgenie")]
    pub opsgenie: Option<crate::datadogV2::model::EntityV3DatadogIntegrationOpsgenie>,
    /// A PagerDuty integration schema.
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<crate::datadogV2::model::EntityV3DatadogIntegrationPagerduty>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3Integrations {
    pub fn new() -> EntityV3Integrations {
        EntityV3Integrations {
            opsgenie: None,
            pagerduty: None,
            _unparsed: false,
        }
    }

    pub fn opsgenie(
        mut self,
        value: crate::datadogV2::model::EntityV3DatadogIntegrationOpsgenie,
    ) -> Self {
        self.opsgenie = Some(value);
        self
    }

    pub fn pagerduty(
        mut self,
        value: crate::datadogV2::model::EntityV3DatadogIntegrationPagerduty,
    ) -> Self {
        self.pagerduty = Some(value);
        self
    }
}

impl Default for EntityV3Integrations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3Integrations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3IntegrationsVisitor;
        impl<'a> Visitor<'a> for EntityV3IntegrationsVisitor {
            type Value = EntityV3Integrations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut opsgenie: Option<
                    crate::datadogV2::model::EntityV3DatadogIntegrationOpsgenie,
                > = None;
                let mut pagerduty: Option<
                    crate::datadogV2::model::EntityV3DatadogIntegrationPagerduty,
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
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3Integrations {
                    opsgenie,
                    pagerduty,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3IntegrationsVisitor)
    }
}
