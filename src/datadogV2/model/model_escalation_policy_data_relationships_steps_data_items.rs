// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a relationship to a single step within an escalation policy. Contains the step's `id` and `type`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyDataRelationshipsStepsDataItems {
    /// Specifies the unique identifier for the step resource.
    #[serde(rename = "id")]
    pub id: String,
    /// Indicates that the resource is of type `steps`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItemsType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyDataRelationshipsStepsDataItems {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItemsType,
    ) -> EscalationPolicyDataRelationshipsStepsDataItems {
        EscalationPolicyDataRelationshipsStepsDataItems {
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

impl<'de> Deserialize<'de> for EscalationPolicyDataRelationshipsStepsDataItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyDataRelationshipsStepsDataItemsVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyDataRelationshipsStepsDataItemsVisitor {
            type Value = EscalationPolicyDataRelationshipsStepsDataItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItemsType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItemsType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = EscalationPolicyDataRelationshipsStepsDataItems {
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyDataRelationshipsStepsDataItemsVisitor)
    }
}
