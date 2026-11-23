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
    /// Optional description of what the routing rule is for. Up to 500 characters. Line breaks are allowed.
    /// Leading and trailing spaces are trimmed, and a blank value is the same as none.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// Optional human-readable name shown in place of the slug. Up to 100 characters on one line. Need not be unique.
    /// Leading and trailing spaces are trimmed, and a blank value is the same as none.
    #[serde(
        rename = "display_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub display_name: Option<Option<String>>,
    /// Reserved for initial View and Edit permissions. Supplying this attribute returns HTTP 400 and doesn't
    /// create the route. Create the route, then set its permissions through the restriction policy API.
    #[serde(rename = "initial_bindings")]
    pub initial_bindings: Option<Vec<crate::datadogV2::model::AiGatewayProviderInitialBinding>>,
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
            description: None,
            display_name: None,
            initial_bindings: None,
            slug,
            tiers,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn initial_bindings(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayProviderInitialBinding>,
    ) -> Self {
        self.initial_bindings = Some(value);
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
                let mut description: Option<Option<String>> = None;
                let mut display_name: Option<Option<String>> = None;
                let mut initial_bindings: Option<
                    Vec<crate::datadogV2::model::AiGatewayProviderInitialBinding>,
                > = None;
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
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initial_bindings" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_bindings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                    description,
                    display_name,
                    initial_bindings,
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
