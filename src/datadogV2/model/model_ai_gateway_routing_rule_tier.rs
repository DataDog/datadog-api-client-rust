// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Routing fallback tier containing targets and their effective ordering.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingRuleTier {
    /// Saved classified-tier settings. Provider and model are empty when the caller cannot view the provider.
    #[serde(rename = "classifier")]
    pub classifier: Option<crate::datadogV2::model::AiGatewayRoutingClassifier>,
    /// How a tier orders its targets.
    #[serde(rename = "ordering")]
    pub ordering: crate::datadogV2::model::AiGatewayRoutingRuleTierOrdering,
    /// Stored tier ordering is inconsistent. mixed means targets have different supported orderings, and unknown means a target has an unsupported ordering. The tier is reported as configured without weights. Replace the rule to repair it. Omitted for consistent tiers. A forced deletion cannot rewrite a surviving tier with an unknown issue.
    #[serde(rename = "ordering_issue")]
    pub ordering_issue: Option<crate::datadogV2::model::AiGatewayRoutingRuleTierOrderingIssue>,
    #[serde(rename = "targets")]
    pub targets: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTarget>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingRuleTier {
    pub fn new(
        ordering: crate::datadogV2::model::AiGatewayRoutingRuleTierOrdering,
        targets: Vec<crate::datadogV2::model::AiGatewayRoutingRuleTarget>,
    ) -> AiGatewayRoutingRuleTier {
        AiGatewayRoutingRuleTier {
            classifier: None,
            ordering,
            ordering_issue: None,
            targets,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn classifier(
        mut self,
        value: crate::datadogV2::model::AiGatewayRoutingClassifier,
    ) -> Self {
        self.classifier = Some(value);
        self
    }

    pub fn ordering_issue(
        mut self,
        value: crate::datadogV2::model::AiGatewayRoutingRuleTierOrderingIssue,
    ) -> Self {
        self.ordering_issue = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRoutingRuleTier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingRuleTierVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingRuleTierVisitor {
            type Value = AiGatewayRoutingRuleTier;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut classifier: Option<crate::datadogV2::model::AiGatewayRoutingClassifier> =
                    None;
                let mut ordering: Option<
                    crate::datadogV2::model::AiGatewayRoutingRuleTierOrdering,
                > = None;
                let mut ordering_issue: Option<
                    crate::datadogV2::model::AiGatewayRoutingRuleTierOrderingIssue,
                > = None;
                let mut targets: Option<Vec<crate::datadogV2::model::AiGatewayRoutingRuleTarget>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "classifier" => {
                            if v.is_null() {
                                continue;
                            }
                            classifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ordering" => {
                            ordering = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _ordering) = ordering {
                                match _ordering {
                                    crate::datadogV2::model::AiGatewayRoutingRuleTierOrdering::UnparsedObject(_ordering) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "ordering_issue" => {
                            if v.is_null() {
                                continue;
                            }
                            ordering_issue =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _ordering_issue) = ordering_issue {
                                match _ordering_issue {
                                    crate::datadogV2::model::AiGatewayRoutingRuleTierOrderingIssue::UnparsedObject(_ordering_issue) => {
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
                let ordering = ordering.ok_or_else(|| M::Error::missing_field("ordering"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;

                let content = AiGatewayRoutingRuleTier {
                    classifier,
                    ordering,
                    ordering_issue,
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRoutingRuleTierVisitor)
    }
}
