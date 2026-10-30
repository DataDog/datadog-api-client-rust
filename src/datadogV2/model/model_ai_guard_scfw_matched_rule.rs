// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One rule the Supply Chain Firewall matched on a package, with the outcome the rule reached.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardScfwMatchedRule {
    /// The Supply Chain Firewall outcome for a package, a matched rule, or the whole request.
    #[serde(rename = "outcome")]
    pub outcome: crate::datadogV2::model::AIGuardScfwOutcome,
    /// The rule text the firewall reported, or the policy type when no text was reported.
    #[serde(rename = "rule")]
    pub rule: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardScfwMatchedRule {
    pub fn new(
        outcome: crate::datadogV2::model::AIGuardScfwOutcome,
        rule: String,
    ) -> AIGuardScfwMatchedRule {
        AIGuardScfwMatchedRule {
            outcome,
            rule,
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

impl<'de> Deserialize<'de> for AIGuardScfwMatchedRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardScfwMatchedRuleVisitor;
        impl<'a> Visitor<'a> for AIGuardScfwMatchedRuleVisitor {
            type Value = AIGuardScfwMatchedRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut outcome: Option<crate::datadogV2::model::AIGuardScfwOutcome> = None;
                let mut rule: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "outcome" => {
                            outcome = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _outcome) = outcome {
                                match _outcome {
                                    crate::datadogV2::model::AIGuardScfwOutcome::UnparsedObject(
                                        _outcome,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "rule" => {
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let outcome = outcome.ok_or_else(|| M::Error::missing_field("outcome"))?;
                let rule = rule.ok_or_else(|| M::Error::missing_field("rule"))?;

                let content = AIGuardScfwMatchedRule {
                    outcome,
                    rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardScfwMatchedRuleVisitor)
    }
}
