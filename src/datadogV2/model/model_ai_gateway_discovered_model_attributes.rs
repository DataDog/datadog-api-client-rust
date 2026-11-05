// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Model discovered from a provider integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayDiscoveredModelAttributes {
    /// Human-readable model name.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Model identifier sent to the provider.
    #[serde(rename = "model_id_wire")]
    pub model_id_wire: String,
    /// Operations supported by the model.
    #[serde(rename = "supported_operations")]
    pub supported_operations: Vec<crate::datadogV2::model::AiGatewaySupportedOperation>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayDiscoveredModelAttributes {
    pub fn new(
        model_id_wire: String,
        supported_operations: Vec<crate::datadogV2::model::AiGatewaySupportedOperation>,
    ) -> AiGatewayDiscoveredModelAttributes {
        AiGatewayDiscoveredModelAttributes {
            display_name: None,
            model_id_wire,
            supported_operations,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayDiscoveredModelAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayDiscoveredModelAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayDiscoveredModelAttributesVisitor {
            type Value = AiGatewayDiscoveredModelAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_name: Option<String> = None;
                let mut model_id_wire: Option<String> = None;
                let mut supported_operations: Option<
                    Vec<crate::datadogV2::model::AiGatewaySupportedOperation>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_id_wire" => {
                            model_id_wire =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "supported_operations" => {
                            supported_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let model_id_wire =
                    model_id_wire.ok_or_else(|| M::Error::missing_field("model_id_wire"))?;
                let supported_operations = supported_operations
                    .ok_or_else(|| M::Error::missing_field("supported_operations"))?;

                let content = AiGatewayDiscoveredModelAttributes {
                    display_name,
                    model_id_wire,
                    supported_operations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayDiscoveredModelAttributesVisitor)
    }
}
