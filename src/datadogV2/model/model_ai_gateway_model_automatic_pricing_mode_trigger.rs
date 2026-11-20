// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Request fields that select a priced mode.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelAutomaticPricingModeTrigger {
    /// Request body fields that must match.
    #[serde(rename = "body")]
    pub body: Option<Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingFieldMatch>>,
    /// Request headers that must match.
    #[serde(rename = "headers")]
    pub headers: Option<Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingFieldMatch>>,
    /// Priced mode selected by the fields.
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayModelAutomaticPricingModeTrigger {
    pub fn new(mode: String) -> AiGatewayModelAutomaticPricingModeTrigger {
        AiGatewayModelAutomaticPricingModeTrigger {
            body: None,
            headers: None,
            mode,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn body(mut self, value: Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingFieldMatch>) -> Self {
        self.body = Some(value);
        self
    }
    
    pub fn headers(mut self, value: Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingFieldMatch>) -> Self {
        self.headers = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayModelAutomaticPricingModeTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelAutomaticPricingModeTriggerVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelAutomaticPricingModeTriggerVisitor {
            type Value = AiGatewayModelAutomaticPricingModeTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut body: Option<Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingFieldMatch>> = None;
                let mut headers: Option<Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingFieldMatch>> = None;
                let mut mode: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "body" => {
                            if v.is_null() {
                                continue;
                            }
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "headers" => {
                            if v.is_null() {
                                continue;
                            }
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = AiGatewayModelAutomaticPricingModeTrigger {
                    body,
                    headers,
                    mode,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelAutomaticPricingModeTriggerVisitor)
    }
}
