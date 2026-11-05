// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom pricing configuration. All four rates are required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingCustomRequestConfig {
    /// Complete set of rates required for custom pricing.
    #[serde(rename = "rates")]
    pub rates: crate::datadogV2::model::AiGatewayModelPricingRequiredRates,
    /// Source for a custom pricing configuration.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::AiGatewayModelPricingCustomRequestSource,
    /// Pricing configuration version. The only supported version is `1`.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingCustomRequestConfig {
    pub fn new(
        rates: crate::datadogV2::model::AiGatewayModelPricingRequiredRates,
        source: crate::datadogV2::model::AiGatewayModelPricingCustomRequestSource,
        version: i64,
    ) -> AiGatewayModelPricingCustomRequestConfig {
        AiGatewayModelPricingCustomRequestConfig {
            rates,
            source,
            version,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingCustomRequestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingCustomRequestConfigVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingCustomRequestConfigVisitor {
            type Value = AiGatewayModelPricingCustomRequestConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rates: Option<crate::datadogV2::model::AiGatewayModelPricingRequiredRates> =
                    None;
                let mut source: Option<
                    crate::datadogV2::model::AiGatewayModelPricingCustomRequestSource,
                > = None;
                let mut version: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rates" => {
                            rates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let rates = rates.ok_or_else(|| M::Error::missing_field("rates"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = AiGatewayModelPricingCustomRequestConfig {
                    rates,
                    source,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingCustomRequestConfigVisitor)
    }
}
