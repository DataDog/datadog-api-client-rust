// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a Page link. A Page can be the child of another Datadog resource
/// (for example, when an incident triggers it), or the parent of one (for example, when the
/// Page is escalated to an incident).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPageLinkAttributes {
    /// The ID of the child resource.
    #[serde(rename = "child_entity_id")]
    pub child_entity_id: String,
    /// The type of a resource linked to a Page.
    #[serde(rename = "child_entity_type")]
    pub child_entity_type: crate::datadogV2::model::OnCallPageLinkEntityType,
    /// The ID of the parent resource.
    #[serde(rename = "parent_entity_id")]
    pub parent_entity_id: String,
    /// The type of a resource linked to a Page.
    #[serde(rename = "parent_entity_type")]
    pub parent_entity_type: crate::datadogV2::model::OnCallPageLinkEntityType,
    /// How the parent resource relates to the child resource.
    #[serde(rename = "relationship")]
    pub relationship: crate::datadogV2::model::OnCallPageLinkRelationshipType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPageLinkAttributes {
    pub fn new(
        child_entity_id: String,
        child_entity_type: crate::datadogV2::model::OnCallPageLinkEntityType,
        parent_entity_id: String,
        parent_entity_type: crate::datadogV2::model::OnCallPageLinkEntityType,
        relationship: crate::datadogV2::model::OnCallPageLinkRelationshipType,
    ) -> OnCallPageLinkAttributes {
        OnCallPageLinkAttributes {
            child_entity_id,
            child_entity_type,
            parent_entity_id,
            parent_entity_type,
            relationship,
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

impl<'de> Deserialize<'de> for OnCallPageLinkAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPageLinkAttributesVisitor;
        impl<'a> Visitor<'a> for OnCallPageLinkAttributesVisitor {
            type Value = OnCallPageLinkAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut child_entity_id: Option<String> = None;
                let mut child_entity_type: Option<
                    crate::datadogV2::model::OnCallPageLinkEntityType,
                > = None;
                let mut parent_entity_id: Option<String> = None;
                let mut parent_entity_type: Option<
                    crate::datadogV2::model::OnCallPageLinkEntityType,
                > = None;
                let mut relationship: Option<
                    crate::datadogV2::model::OnCallPageLinkRelationshipType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "child_entity_id" => {
                            child_entity_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "child_entity_type" => {
                            child_entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _child_entity_type) = child_entity_type {
                                match _child_entity_type {
                                    crate::datadogV2::model::OnCallPageLinkEntityType::UnparsedObject(_child_entity_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "parent_entity_id" => {
                            parent_entity_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_entity_type" => {
                            parent_entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _parent_entity_type) = parent_entity_type {
                                match _parent_entity_type {
                                    crate::datadogV2::model::OnCallPageLinkEntityType::UnparsedObject(_parent_entity_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "relationship" => {
                            relationship =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _relationship) = relationship {
                                match _relationship {
                                    crate::datadogV2::model::OnCallPageLinkRelationshipType::UnparsedObject(_relationship) => {
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
                let child_entity_id =
                    child_entity_id.ok_or_else(|| M::Error::missing_field("child_entity_id"))?;
                let child_entity_type = child_entity_type
                    .ok_or_else(|| M::Error::missing_field("child_entity_type"))?;
                let parent_entity_id =
                    parent_entity_id.ok_or_else(|| M::Error::missing_field("parent_entity_id"))?;
                let parent_entity_type = parent_entity_type
                    .ok_or_else(|| M::Error::missing_field("parent_entity_type"))?;
                let relationship =
                    relationship.ok_or_else(|| M::Error::missing_field("relationship"))?;

                let content = OnCallPageLinkAttributes {
                    child_entity_id,
                    child_entity_type,
                    parent_entity_id,
                    parent_entity_type,
                    relationship,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPageLinkAttributesVisitor)
    }
}
