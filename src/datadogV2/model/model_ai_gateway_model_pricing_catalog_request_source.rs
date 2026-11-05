// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Source for a catalog pricing configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingCatalogRequestSource {
    /// Pricing catalog provider identifier.
    #[serde(rename = "provider_id")]
    pub provider_id: String,
    /// Catalog pricing source type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSourceType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingCatalogRequestSource {
    pub fn new(
        provider_id: String,
        type_: crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSourceType,
    ) -> AiGatewayModelPricingCatalogRequestSource {
        AiGatewayModelPricingCatalogRequestSource {
            provider_id,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingCatalogRequestSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingCatalogRequestSourceVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingCatalogRequestSourceVisitor {
            type Value = AiGatewayModelPricingCatalogRequestSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut provider_id: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSourceType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "provider_id" => {
                            provider_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let provider_id =
                    provider_id.ok_or_else(|| M::Error::missing_field("provider_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AiGatewayModelPricingCatalogRequestSource {
                    provider_id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingCatalogRequestSourceVisitor)
    }
}
