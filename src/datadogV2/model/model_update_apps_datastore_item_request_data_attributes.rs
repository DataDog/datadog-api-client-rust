// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a datastore item, including the item key and changes to apply.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateAppsDatastoreItemRequestDataAttributes {
    /// The unique identifier of the item being updated.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Changes to apply to a datastore item using set operations.
    #[serde(rename = "item_changes")]
    pub item_changes:
        crate::datadogV2::model::UpdateAppsDatastoreItemRequestDataAttributesItemChanges,
    /// The primary key that identifies the item to update. Cannot exceed 256 characters.
    #[serde(rename = "item_key")]
    pub item_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateAppsDatastoreItemRequestDataAttributes {
    pub fn new(
        item_changes: crate::datadogV2::model::UpdateAppsDatastoreItemRequestDataAttributesItemChanges,
        item_key: String,
    ) -> UpdateAppsDatastoreItemRequestDataAttributes {
        UpdateAppsDatastoreItemRequestDataAttributes {
            id: None,
            item_changes,
            item_key,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for UpdateAppsDatastoreItemRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateAppsDatastoreItemRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateAppsDatastoreItemRequestDataAttributesVisitor {
            type Value = UpdateAppsDatastoreItemRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut item_changes: Option<crate::datadogV2::model::UpdateAppsDatastoreItemRequestDataAttributesItemChanges> = None;
                let mut item_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "item_changes" => {
                            item_changes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "item_key" => {
                            item_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let item_changes =
                    item_changes.ok_or_else(|| M::Error::missing_field("item_changes"))?;
                let item_key = item_key.ok_or_else(|| M::Error::missing_field("item_key"))?;

                let content = UpdateAppsDatastoreItemRequestDataAttributes {
                    id,
                    item_changes,
                    item_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateAppsDatastoreItemRequestDataAttributesVisitor)
    }
}
