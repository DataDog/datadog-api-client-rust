// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The automatically synced published price. Present only when it is the effective price.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelAutomaticPricing {
    /// Time the price was last revalidated against the source.
    #[serde(rename = "last_validated_at")]
    pub last_validated_at: chrono::DateTime<chrono::Utc>,
    /// The reviewed endpoint mapping used to identify a custom provider. Omitted when the provider and model identity matched exactly.
    #[serde(rename = "mapping")]
    pub mapping: Option<crate::datadogV2::model::AiGatewayModelAutomaticPricingMapping>,
    /// Request fields that select each priced mode, such as `speed: fast` for `anthropic` models. Omitted when the model has a single mode.
    #[serde(rename = "mode_triggers")]
    pub mode_triggers:
        Option<Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingModeTrigger>>,
    /// Time the price was observed in the source.
    #[serde(rename = "observed_at")]
    pub observed_at: chrono::DateTime<chrono::Utc>,
    /// Published rates.
    #[serde(rename = "rates")]
    pub rates: Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingRate>,
    /// The pricing source entry the price was read from.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::AiGatewayModelAutomaticPricingSource,
    /// True when the price has not been revalidated recently and is being retained as last-known-good.
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelAutomaticPricing {
    pub fn new(
        last_validated_at: chrono::DateTime<chrono::Utc>,
        observed_at: chrono::DateTime<chrono::Utc>,
        rates: Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingRate>,
        source: crate::datadogV2::model::AiGatewayModelAutomaticPricingSource,
        stale: bool,
    ) -> AiGatewayModelAutomaticPricing {
        AiGatewayModelAutomaticPricing {
            last_validated_at,
            mapping: None,
            mode_triggers: None,
            observed_at,
            rates,
            source,
            stale,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn mapping(
        mut self,
        value: crate::datadogV2::model::AiGatewayModelAutomaticPricingMapping,
    ) -> Self {
        self.mapping = Some(value);
        self
    }

    pub fn mode_triggers(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingModeTrigger>,
    ) -> Self {
        self.mode_triggers = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayModelAutomaticPricing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelAutomaticPricingVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelAutomaticPricingVisitor {
            type Value = AiGatewayModelAutomaticPricing;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut last_validated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut mapping: Option<
                    crate::datadogV2::model::AiGatewayModelAutomaticPricingMapping,
                > = None;
                let mut mode_triggers: Option<
                    Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingModeTrigger>,
                > = None;
                let mut observed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut rates: Option<
                    Vec<crate::datadogV2::model::AiGatewayModelAutomaticPricingRate>,
                > = None;
                let mut source: Option<
                    crate::datadogV2::model::AiGatewayModelAutomaticPricingSource,
                > = None;
                let mut stale: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "last_validated_at" => {
                            last_validated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapping" => {
                            if v.is_null() {
                                continue;
                            }
                            mapping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode_triggers" => {
                            if v.is_null() {
                                continue;
                            }
                            mode_triggers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observed_at" => {
                            observed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rates" => {
                            rates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stale" => {
                            stale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let last_validated_at = last_validated_at
                    .ok_or_else(|| M::Error::missing_field("last_validated_at"))?;
                let observed_at =
                    observed_at.ok_or_else(|| M::Error::missing_field("observed_at"))?;
                let rates = rates.ok_or_else(|| M::Error::missing_field("rates"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let stale = stale.ok_or_else(|| M::Error::missing_field("stale"))?;

                let content = AiGatewayModelAutomaticPricing {
                    last_validated_at,
                    mapping,
                    mode_triggers,
                    observed_at,
                    rates,
                    source,
                    stale,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelAutomaticPricingVisitor)
    }
}
