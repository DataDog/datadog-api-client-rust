// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Changes to apply to a datastore item using set operations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
    /// Set operation that contains key-value pairs to set on the datastore item.
    #[serde(rename = "ops_set")]
    pub ops_set: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
    pub fn new() -> UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
        UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
            ops_set: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ops_set(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.ops_set = Some(value);
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

impl Default for UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateAppsDatastoreItemRequestDataAttributesItemChangesVisitor;
        impl<'a> Visitor<'a> for UpdateAppsDatastoreItemRequestDataAttributesItemChangesVisitor {
            type Value = UpdateAppsDatastoreItemRequestDataAttributesItemChanges;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ops_set: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ops_set" => {
                            if v.is_null() {
                                continue;
                            }
                            ops_set = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateAppsDatastoreItemRequestDataAttributesItemChanges {
                    ops_set,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateAppsDatastoreItemRequestDataAttributesItemChangesVisitor)
    }
}
