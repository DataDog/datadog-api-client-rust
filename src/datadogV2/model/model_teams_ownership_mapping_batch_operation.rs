// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single add or remove operation, applied atomically with every other operation in the request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingBatchOperation {
    /// The mapping to add. Required when `op` is `add`.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::TeamsOwnershipMappingBatchOperationData>,
    /// Whether this operation adds a new mapping or removes an existing one.
    #[serde(rename = "op")]
    pub op: crate::datadogV2::model::TeamsOwnershipMappingBatchOperationOp,
    /// Identifies an existing mapping to remove. Required when `op` is `remove`.
    #[serde(rename = "ref")]
    pub ref_: Option<crate::datadogV2::model::TeamsOwnershipMappingBatchOperationRef>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipMappingBatchOperation {
    pub fn new(
        op: crate::datadogV2::model::TeamsOwnershipMappingBatchOperationOp,
    ) -> TeamsOwnershipMappingBatchOperation {
        TeamsOwnershipMappingBatchOperation {
            data: None,
            op,
            ref_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: crate::datadogV2::model::TeamsOwnershipMappingBatchOperationData,
    ) -> Self {
        self.data = Some(value);
        self
    }

    pub fn ref_(
        mut self,
        value: crate::datadogV2::model::TeamsOwnershipMappingBatchOperationRef,
    ) -> Self {
        self.ref_ = Some(value);
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

impl<'de> Deserialize<'de> for TeamsOwnershipMappingBatchOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingBatchOperationVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingBatchOperationVisitor {
            type Value = TeamsOwnershipMappingBatchOperation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    crate::datadogV2::model::TeamsOwnershipMappingBatchOperationData,
                > = None;
                let mut op: Option<crate::datadogV2::model::TeamsOwnershipMappingBatchOperationOp> =
                    None;
                let mut ref_: Option<
                    crate::datadogV2::model::TeamsOwnershipMappingBatchOperationRef,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "op" => {
                            op = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _op) = op {
                                match _op {
                                    crate::datadogV2::model::TeamsOwnershipMappingBatchOperationOp::UnparsedObject(_op) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "ref" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let op = op.ok_or_else(|| M::Error::missing_field("op"))?;

                let content = TeamsOwnershipMappingBatchOperation {
                    data,
                    op,
                    ref_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipMappingBatchOperationVisitor)
    }
}
