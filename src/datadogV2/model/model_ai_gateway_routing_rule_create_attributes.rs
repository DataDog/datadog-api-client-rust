// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create an AI Gateway routing rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingRuleCreateAttributes {
    /// Organization-unique resource slug.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "tiers")]
    pub tiers: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTierRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingRuleCreateAttributes {
    pub fn new(
        slug: String,
        tiers: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTierRequest>,
    ) -> AiGatewayRoutingRuleCreateAttributes {
        AiGatewayRoutingRuleCreateAttributes {
            slug,
            tiers,
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

impl<'de> Deserialize<'de> for AiGatewayRoutingRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingRuleCreateAttributesVisitor {
            type Value = AiGatewayRoutingRuleCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut slug: Option<String> = None;
                let mut tiers: Option<
                    Vec<crate::datadogV2::model::AiGatewayRoutingRuleTierRequest>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "slug" => {
                            slug = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tiers" => {
                            tiers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let slug = slug.ok_or_else(|| M::Error::missing_field("slug"))?;
                let tiers = tiers.ok_or_else(|| M::Error::missing_field("tiers"))?;

                let content = AiGatewayRoutingRuleCreateAttributes {
                    slug,
                    tiers,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRoutingRuleCreateAttributesVisitor)
    }
}
