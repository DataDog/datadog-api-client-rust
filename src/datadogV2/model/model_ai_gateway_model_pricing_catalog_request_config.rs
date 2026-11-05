// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Catalog pricing configuration with optional manual rate overrides.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingCatalogRequestConfig {
    /// Pricing rates for a model.
    #[serde(rename = "overrides")]
    pub overrides: Option<crate::datadogV2::model::AiGatewayModelPricingRates>,
    /// Source for a catalog pricing configuration.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSource,
    /// Pricing configuration version. The only supported version is `1`.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingCatalogRequestConfig {
    pub fn new(
        source: crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSource,
        version: i64,
    ) -> AiGatewayModelPricingCatalogRequestConfig {
        AiGatewayModelPricingCatalogRequestConfig {
            overrides: None,
            source,
            version,
            _unparsed: false,
        }
    }

    pub fn overrides(mut self, value: crate::datadogV2::model::AiGatewayModelPricingRates) -> Self {
        self.overrides = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingCatalogRequestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingCatalogRequestConfigVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingCatalogRequestConfigVisitor {
            type Value = AiGatewayModelPricingCatalogRequestConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut overrides: Option<crate::datadogV2::model::AiGatewayModelPricingRates> =
                    None;
                let mut source: Option<
                    crate::datadogV2::model::AiGatewayModelPricingCatalogRequestSource,
                > = None;
                let mut version: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "overrides" => {
                            if v.is_null() {
                                continue;
                            }
                            overrides = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = AiGatewayModelPricingCatalogRequestConfig {
                    overrides,
                    source,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingCatalogRequestConfigVisitor)
    }
}
