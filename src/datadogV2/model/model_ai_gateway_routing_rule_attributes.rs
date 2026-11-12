// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Routing rule configuration and metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingRuleAttributes {
    /// Time when the resource was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Stable numeric route identifier used by scoped policies and `ai-gateway-route` restriction policies.
    #[serde(rename = "route_id")]
    pub route_id: i64,
    #[serde(rename = "tiers")]
    pub tiers: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTier>,
    /// Time when the resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingRuleAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        route_id: i64,
        tiers: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTier>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayRoutingRuleAttributes {
        AiGatewayRoutingRuleAttributes {
            created_at,
            route_id,
            tiers,
            updated_at,
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

impl<'de> Deserialize<'de> for AiGatewayRoutingRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingRuleAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingRuleAttributesVisitor {
            type Value = AiGatewayRoutingRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut route_id: Option<i64> = None;
                let mut tiers: Option<Vec<crate::datadogV2::model::AiGatewayRoutingRuleTier>> =
                    None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "route_id" => {
                            route_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tiers" => {
                            tiers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let route_id = route_id.ok_or_else(|| M::Error::missing_field("route_id"))?;
                let tiers = tiers.ok_or_else(|| M::Error::missing_field("tiers"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AiGatewayRoutingRuleAttributes {
                    created_at,
                    route_id,
                    tiers,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRoutingRuleAttributesVisitor)
    }
}
