// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Pricing configuration accepted when setting model pricing.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AiGatewayModelPricingRequestConfig {
    AiGatewayModelPricingCustomRequestConfig(
        Box<crate::datadogV2::model::AiGatewayModelPricingCustomRequestConfig>,
    ),
    AiGatewayModelPricingCatalogRequestConfig(
        Box<crate::datadogV2::model::AiGatewayModelPricingCatalogRequestConfig>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingRequestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AiGatewayModelPricingCustomRequestConfig>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    AiGatewayModelPricingRequestConfig::AiGatewayModelPricingCustomRequestConfig(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AiGatewayModelPricingCatalogRequestConfig>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    AiGatewayModelPricingRequestConfig::AiGatewayModelPricingCatalogRequestConfig(
                        _v,
                    ),
                );
            }
        }

        return Ok(AiGatewayModelPricingRequestConfig::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
