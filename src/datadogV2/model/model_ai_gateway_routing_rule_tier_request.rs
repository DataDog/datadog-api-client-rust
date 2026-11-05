// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Routing fallback tier used to create or replace a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingRuleTierRequest {
    /// How a tier orders its targets. Omitted or null defaults to configured, which preserves target order. The weighted_affinity ordering distributes first-choice assignments by relative target weights and accepts only model targets with an explicit provider, without repeated provider and model pairs in the same tier.
    #[serde(
        rename = "ordering",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ordering: Option<Option<crate::datadogV2::model::AiGatewayNullableRoutingRuleTierOrdering>>,
    #[serde(rename = "targets")]
    pub targets: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTargetRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingRuleTierRequest {
    pub fn new(
        targets: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTargetRequest>,
    ) -> AiGatewayRoutingRuleTierRequest {
        AiGatewayRoutingRuleTierRequest {
            ordering: None,
            targets,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ordering(
        mut self,
        value: Option<crate::datadogV2::model::AiGatewayNullableRoutingRuleTierOrdering>,
    ) -> Self {
        self.ordering = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRoutingRuleTierRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingRuleTierRequestVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingRuleTierRequestVisitor {
            type Value = AiGatewayRoutingRuleTierRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ordering: Option<
                    Option<crate::datadogV2::model::AiGatewayNullableRoutingRuleTierOrdering>,
                > = None;
                let mut targets: Option<
                    Vec<crate::datadogV2::model::AiGatewayRoutingRuleTargetRequest>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ordering" => {
                            ordering = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _ordering) = ordering {
                                match _ordering {
                                    Some(crate::datadogV2::model::AiGatewayNullableRoutingRuleTierOrdering::UnparsedObject(_ordering)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;

                let content = AiGatewayRoutingRuleTierRequest {
                    ordering,
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRoutingRuleTierRequestVisitor)
    }
}
