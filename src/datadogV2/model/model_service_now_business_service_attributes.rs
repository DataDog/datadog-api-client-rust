// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a ServiceNow business service
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceNowBusinessServiceAttributes {
    /// The ID of the ServiceNow instance
    #[serde(rename = "instance_id")]
    pub instance_id: uuid::Uuid,
    /// The name of the business service
    #[serde(rename = "service_name")]
    pub service_name: String,
    /// The system ID of the business service in ServiceNow
    #[serde(rename = "service_sys_id")]
    pub service_sys_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceNowBusinessServiceAttributes {
    pub fn new(
        instance_id: uuid::Uuid,
        service_name: String,
        service_sys_id: String,
    ) -> ServiceNowBusinessServiceAttributes {
        ServiceNowBusinessServiceAttributes {
            instance_id,
            service_name,
            service_sys_id,
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

impl<'de> Deserialize<'de> for ServiceNowBusinessServiceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceNowBusinessServiceAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceNowBusinessServiceAttributesVisitor {
            type Value = ServiceNowBusinessServiceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut instance_id: Option<uuid::Uuid> = None;
                let mut service_name: Option<String> = None;
                let mut service_sys_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "instance_id" => {
                            instance_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_sys_id" => {
                            service_sys_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let instance_id =
                    instance_id.ok_or_else(|| M::Error::missing_field("instance_id"))?;
                let service_name =
                    service_name.ok_or_else(|| M::Error::missing_field("service_name"))?;
                let service_sys_id =
                    service_sys_id.ok_or_else(|| M::Error::missing_field("service_sys_id"))?;

                let content = ServiceNowBusinessServiceAttributes {
                    instance_id,
                    service_name,
                    service_sys_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceNowBusinessServiceAttributesVisitor)
    }
}
