// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Provider entries grouped under one canonical model ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelsByCanonicalAttributes {
    /// Provider-independent model identifier.
    #[serde(rename = "model_id_canonical")]
    pub model_id_canonical: String,
    #[serde(rename = "providers")]
    pub providers: Vec<crate::datadogV2::model::AiGatewayProviderModelEntry>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelsByCanonicalAttributes {
    pub fn new(
        model_id_canonical: String,
        providers: Vec<crate::datadogV2::model::AiGatewayProviderModelEntry>,
    ) -> AiGatewayModelsByCanonicalAttributes {
        AiGatewayModelsByCanonicalAttributes {
            model_id_canonical,
            providers,
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

impl<'de> Deserialize<'de> for AiGatewayModelsByCanonicalAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelsByCanonicalAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelsByCanonicalAttributesVisitor {
            type Value = AiGatewayModelsByCanonicalAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut model_id_canonical: Option<String> = None;
                let mut providers: Option<
                    Vec<crate::datadogV2::model::AiGatewayProviderModelEntry>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "model_id_canonical" => {
                            model_id_canonical =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "providers" => {
                            providers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let model_id_canonical = model_id_canonical
                    .ok_or_else(|| M::Error::missing_field("model_id_canonical"))?;
                let providers = providers.ok_or_else(|| M::Error::missing_field("providers"))?;

                let content = AiGatewayModelsByCanonicalAttributes {
                    model_id_canonical,
                    providers,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelsByCanonicalAttributesVisitor)
    }
}
