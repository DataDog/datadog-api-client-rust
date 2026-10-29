// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data of the started rule proposal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleProposalStartResponseData {
    /// Attributes of the started rule proposal.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SecurityMonitoringRuleProposalStartResponseAttributes,
    /// The ID of the rule proposal. Use it as `proposal_id` to check the proposal status and get the proposed rule.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The rule proposal resource type. The value is always `rule_proposals`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityMonitoringRuleProposalType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleProposalStartResponseData {
    pub fn new(
        attributes: crate::datadogV2::model::SecurityMonitoringRuleProposalStartResponseAttributes,
        id: uuid::Uuid,
        type_: crate::datadogV2::model::SecurityMonitoringRuleProposalType,
    ) -> SecurityMonitoringRuleProposalStartResponseData {
        SecurityMonitoringRuleProposalStartResponseData {
            attributes,
            id,
            type_,
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleProposalStartResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleProposalStartResponseDataVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleProposalStartResponseDataVisitor {
            type Value = SecurityMonitoringRuleProposalStartResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleProposalStartResponseAttributes,
                > = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut type_: Option<crate::datadogV2::model::SecurityMonitoringRuleProposalType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityMonitoringRuleProposalType::UnparsedObject(_type_) => {
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
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringRuleProposalStartResponseData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleProposalStartResponseDataVisitor)
    }
}
