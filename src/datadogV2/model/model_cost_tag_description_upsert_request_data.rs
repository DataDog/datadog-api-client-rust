// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resource envelope carrying the tag key description being upserted. The `id` is informational; the authoritative tag key is taken from the URL path.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostTagDescriptionUpsertRequestData {
    /// Mutable attributes set when creating or updating a Cloud Cost Management tag key description.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::CostTagDescriptionUpsertRequestDataAttributes,
    /// Identifier of the tag key the description applies to. Matches the `tag_key` path parameter.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of the Cloud Cost Management tag description resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CostTagDescriptionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostTagDescriptionUpsertRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::CostTagDescriptionUpsertRequestDataAttributes,
        type_: crate::datadogV2::model::CostTagDescriptionType,
    ) -> CostTagDescriptionUpsertRequestData {
        CostTagDescriptionUpsertRequestData {
            attributes,
            id: None,
            type_,
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

impl<'de> Deserialize<'de> for CostTagDescriptionUpsertRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostTagDescriptionUpsertRequestDataVisitor;
        impl<'a> Visitor<'a> for CostTagDescriptionUpsertRequestDataVisitor {
            type Value = CostTagDescriptionUpsertRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::CostTagDescriptionUpsertRequestDataAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CostTagDescriptionType> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CostTagDescriptionType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CostTagDescriptionUpsertRequestData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostTagDescriptionUpsertRequestDataVisitor)
    }
}
