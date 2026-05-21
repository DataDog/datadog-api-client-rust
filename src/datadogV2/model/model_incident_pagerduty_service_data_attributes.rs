// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a PagerDuty service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentPagerdutyServiceDataAttributes {
    /// The handle for the PagerDuty service.
    #[serde(rename = "handle")]
    pub handle: String,
    /// The name of the PagerDuty service.
    #[serde(rename = "name")]
    pub name: String,
    /// The PagerDuty service identifier.
    #[serde(rename = "service_id")]
    pub service_id: String,
    /// Whether webhooks are enabled for this service.
    #[serde(rename = "webhooks_enabled")]
    pub webhooks_enabled: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentPagerdutyServiceDataAttributes {
    pub fn new(
        handle: String,
        name: String,
        service_id: String,
        webhooks_enabled: bool,
    ) -> IncidentPagerdutyServiceDataAttributes {
        IncidentPagerdutyServiceDataAttributes {
            handle,
            name,
            service_id,
            webhooks_enabled,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentPagerdutyServiceDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentPagerdutyServiceDataAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentPagerdutyServiceDataAttributesVisitor {
            type Value = IncidentPagerdutyServiceDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut handle: Option<String> = None;
                let mut name: Option<String> = None;
                let mut service_id: Option<String> = None;
                let mut webhooks_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_id" => {
                            service_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "webhooks_enabled" => {
                            webhooks_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let service_id = service_id.ok_or_else(|| M::Error::missing_field("service_id"))?;
                let webhooks_enabled =
                    webhooks_enabled.ok_or_else(|| M::Error::missing_field("webhooks_enabled"))?;

                let content = IncidentPagerdutyServiceDataAttributes {
                    handle,
                    name,
                    service_id,
                    webhooks_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentPagerdutyServiceDataAttributesVisitor)
    }
}
