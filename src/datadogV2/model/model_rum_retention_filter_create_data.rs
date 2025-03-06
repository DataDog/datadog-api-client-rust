// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The new RUM retention filter properties to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRetentionFilterCreateData {
    /// The object describing attributes of a RUM retention filter to create.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::RumRetentionFilterCreateAttributes,
    /// The object describing metadata of a RUM retention filter.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::RumRetentionFilterMeta>,
    /// The type of the resource. The value should always be retention_filters.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RumRetentionFilterType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRetentionFilterCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::RumRetentionFilterCreateAttributes,
        type_: crate::datadogV2::model::RumRetentionFilterType,
    ) -> RumRetentionFilterCreateData {
        RumRetentionFilterCreateData {
            attributes,
            meta: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::RumRetentionFilterMeta) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for RumRetentionFilterCreateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRetentionFilterCreateDataVisitor;
        impl<'a> Visitor<'a> for RumRetentionFilterCreateDataVisitor {
            type Value = RumRetentionFilterCreateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::RumRetentionFilterCreateAttributes,
                > = None;
                let mut meta: Option<crate::datadogV2::model::RumRetentionFilterMeta> = None;
                let mut type_: Option<crate::datadogV2::model::RumRetentionFilterType> = None;
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
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RumRetentionFilterType::UnparsedObject(_type_) => {
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

                let content = RumRetentionFilterCreateData {
                    attributes,
                    meta,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRetentionFilterCreateDataVisitor)
    }
}
