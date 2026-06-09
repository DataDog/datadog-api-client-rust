// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The data wrapper for a facet info response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmHostFacetInfoData {
    /// Attributes of a facet info response, containing the value distribution for the requested facet.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::CsmHostFacetInfoAttributes,
    /// The identifier of the facet.
    #[serde(rename = "id")]
    pub id: String,
    /// Metadata for the facet info response.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::CsmHostFacetInfoMeta,
    /// The JSON:API type for facet info resources. The value should always be `facet_info`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CsmFacetInfoType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmHostFacetInfoData {
    pub fn new(
        attributes: crate::datadogV2::model::CsmHostFacetInfoAttributes,
        id: String,
        meta: crate::datadogV2::model::CsmHostFacetInfoMeta,
        type_: crate::datadogV2::model::CsmFacetInfoType,
    ) -> CsmHostFacetInfoData {
        CsmHostFacetInfoData {
            attributes,
            id,
            meta,
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

impl<'de> Deserialize<'de> for CsmHostFacetInfoData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmHostFacetInfoDataVisitor;
        impl<'a> Visitor<'a> for CsmHostFacetInfoDataVisitor {
            type Value = CsmHostFacetInfoData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::CsmHostFacetInfoAttributes> =
                    None;
                let mut id: Option<String> = None;
                let mut meta: Option<crate::datadogV2::model::CsmHostFacetInfoMeta> = None;
                let mut type_: Option<crate::datadogV2::model::CsmFacetInfoType> = None;
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
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CsmFacetInfoType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CsmHostFacetInfoData {
                    attributes,
                    id,
                    meta,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmHostFacetInfoDataVisitor)
    }
}
