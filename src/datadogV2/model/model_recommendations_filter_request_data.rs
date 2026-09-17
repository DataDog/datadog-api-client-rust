// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON:API resource containing the cost recommendations filter. This legacy search contract
/// uses the resource ID for the filter expression rather than as a persistent resource identifier.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecommendationsFilterRequestData {
    /// Attributes used to filter and sort cost recommendations.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::RecommendationsFilterRequestDataAttributes>,
    /// Filter expression applied to the recommendations. The server treats an omitted ID as `*`
    /// and returns all recommendations.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Legacy JSON:API resource type required by the cost recommendations search decoder.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RecommendationsFilterRequestDataType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RecommendationsFilterRequestData {
    pub fn new(
        type_: crate::datadogV2::model::RecommendationsFilterRequestDataType,
    ) -> RecommendationsFilterRequestData {
        RecommendationsFilterRequestData {
            attributes: None,
            id: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::RecommendationsFilterRequestDataAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
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

impl<'de> Deserialize<'de> for RecommendationsFilterRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecommendationsFilterRequestDataVisitor;
        impl<'a> Visitor<'a> for RecommendationsFilterRequestDataVisitor {
            type Value = RecommendationsFilterRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::RecommendationsFilterRequestDataAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::RecommendationsFilterRequestDataType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
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
                                    crate::datadogV2::model::RecommendationsFilterRequestDataType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = RecommendationsFilterRequestData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RecommendationsFilterRequestDataVisitor)
    }
}
