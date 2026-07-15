// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an org group policy suggestion.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicySuggestionAttributes {
    /// The ratio of member orgs whose configuration agrees on the recommended value.
    #[serde(rename = "consensus_ratio")]
    pub consensus_ratio: f64,
    /// The name of the suggested policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    /// The recommended value for the policy, based on member org consensus.
    #[serde(rename = "recommended_value")]
    pub recommended_value: serde_json::Value,
    /// The status of the policy suggestion.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::OrgGroupPolicySuggestionStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicySuggestionAttributes {
    pub fn new(
        consensus_ratio: f64,
        policy_name: String,
        recommended_value: serde_json::Value,
        status: crate::datadogV2::model::OrgGroupPolicySuggestionStatus,
    ) -> OrgGroupPolicySuggestionAttributes {
        OrgGroupPolicySuggestionAttributes {
            consensus_ratio,
            policy_name,
            recommended_value,
            status,
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

impl<'de> Deserialize<'de> for OrgGroupPolicySuggestionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicySuggestionAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicySuggestionAttributesVisitor {
            type Value = OrgGroupPolicySuggestionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut consensus_ratio: Option<f64> = None;
                let mut policy_name: Option<String> = None;
                let mut recommended_value: Option<serde_json::Value> = None;
                let mut status: Option<crate::datadogV2::model::OrgGroupPolicySuggestionStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "consensus_ratio" => {
                            consensus_ratio =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_name" => {
                            policy_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommended_value" => {
                            recommended_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::OrgGroupPolicySuggestionStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let consensus_ratio =
                    consensus_ratio.ok_or_else(|| M::Error::missing_field("consensus_ratio"))?;
                let policy_name =
                    policy_name.ok_or_else(|| M::Error::missing_field("policy_name"))?;
                let recommended_value = recommended_value
                    .ok_or_else(|| M::Error::missing_field("recommended_value"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = OrgGroupPolicySuggestionAttributes {
                    consensus_ratio,
                    policy_name,
                    recommended_value,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicySuggestionAttributesVisitor)
    }
}
