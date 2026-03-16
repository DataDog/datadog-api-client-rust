// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Identifies a team or organization hierarchy to include in synchronization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamSyncSelectionStateItem {
    /// The external identifier for a team or organization in the source platform.
    #[serde(rename = "external_id")]
    pub external_id: crate::datadogV2::model::TeamSyncSelectionStateExternalId,
    /// The operation to perform on the selected hierarchy.
    /// When set to `include`, synchronization covers the
    /// referenced teams or organizations.
    #[serde(rename = "operation")]
    pub operation: Option<crate::datadogV2::model::TeamSyncSelectionStateOperation>,
    /// The scope of the selection. When set to `subtree`,
    /// synchronization includes the referenced team or
    /// organization and everything nested under it.
    #[serde(rename = "scope")]
    pub scope: Option<crate::datadogV2::model::TeamSyncSelectionStateScope>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamSyncSelectionStateItem {
    pub fn new(
        external_id: crate::datadogV2::model::TeamSyncSelectionStateExternalId,
    ) -> TeamSyncSelectionStateItem {
        TeamSyncSelectionStateItem {
            external_id,
            operation: None,
            scope: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn operation(
        mut self,
        value: crate::datadogV2::model::TeamSyncSelectionStateOperation,
    ) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn scope(mut self, value: crate::datadogV2::model::TeamSyncSelectionStateScope) -> Self {
        self.scope = Some(value);
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

impl<'de> Deserialize<'de> for TeamSyncSelectionStateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamSyncSelectionStateItemVisitor;
        impl<'a> Visitor<'a> for TeamSyncSelectionStateItemVisitor {
            type Value = TeamSyncSelectionStateItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut external_id: Option<
                    crate::datadogV2::model::TeamSyncSelectionStateExternalId,
                > = None;
                let mut operation: Option<
                    crate::datadogV2::model::TeamSyncSelectionStateOperation,
                > = None;
                let mut scope: Option<crate::datadogV2::model::TeamSyncSelectionStateScope> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "external_id" => {
                            external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation" => {
                            if v.is_null() {
                                continue;
                            }
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operation) = operation {
                                match _operation {
                                    crate::datadogV2::model::TeamSyncSelectionStateOperation::UnparsedObject(_operation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scope) = scope {
                                match _scope {
                                    crate::datadogV2::model::TeamSyncSelectionStateScope::UnparsedObject(_scope) => {
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
                let external_id =
                    external_id.ok_or_else(|| M::Error::missing_field("external_id"))?;

                let content = TeamSyncSelectionStateItem {
                    external_id,
                    operation,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamSyncSelectionStateItemVisitor)
    }
}
