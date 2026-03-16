// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The external identifier for a team or organization in the source platform.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamSyncSelectionStateExternalId {
    /// The type of external identifier for the selection state item.
    /// For GitHub synchronization, the allowed values are `team` and
    /// `organization`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamSyncSelectionStateExternalIdType,
    /// The external identifier value from the source
    /// platform. For GitHub, this is the string
    /// representation of a GitHub organization ID or team
    /// ID.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamSyncSelectionStateExternalId {
    pub fn new(
        type_: crate::datadogV2::model::TeamSyncSelectionStateExternalIdType,
        value: String,
    ) -> TeamSyncSelectionStateExternalId {
        TeamSyncSelectionStateExternalId {
            type_,
            value,
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

impl<'de> Deserialize<'de> for TeamSyncSelectionStateExternalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamSyncSelectionStateExternalIdVisitor;
        impl<'a> Visitor<'a> for TeamSyncSelectionStateExternalIdVisitor {
            type Value = TeamSyncSelectionStateExternalId;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<
                    crate::datadogV2::model::TeamSyncSelectionStateExternalIdType,
                > = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TeamSyncSelectionStateExternalIdType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = TeamSyncSelectionStateExternalId {
                    type_,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamSyncSelectionStateExternalIdVisitor)
    }
}
