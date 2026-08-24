// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The request body for bulk-creating and bulk-removing teams ownership mappings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingBatchRequest {
    /// The list of add and remove operations to apply atomically.
    #[serde(rename = "atomic:operations")]
    pub atomic_operations: Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchOperation>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipMappingBatchRequest {
    pub fn new(
        atomic_operations: Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchOperation>,
    ) -> TeamsOwnershipMappingBatchRequest {
        TeamsOwnershipMappingBatchRequest {
            atomic_operations,
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

impl<'de> Deserialize<'de> for TeamsOwnershipMappingBatchRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingBatchRequestVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingBatchRequestVisitor {
            type Value = TeamsOwnershipMappingBatchRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut atomic_operations: Option<
                    Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchOperation>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "atomic:operations" => {
                            atomic_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let atomic_operations = atomic_operations
                    .ok_or_else(|| M::Error::missing_field("atomic_operations"))?;

                let content = TeamsOwnershipMappingBatchRequest {
                    atomic_operations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipMappingBatchRequestVisitor)
    }
}
