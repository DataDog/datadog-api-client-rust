// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the IoC Explorer list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IoCExplorerListResponseAttributes {
    /// List of indicators of compromise.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::IoCIndicator>>,
    /// Response metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::IoCExplorerListResponseMetadata>,
    /// Pagination information.
    #[serde(rename = "paging")]
    pub paging: Option<crate::datadogV2::model::IoCExplorerListResponsePaging>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IoCExplorerListResponseAttributes {
    pub fn new() -> IoCExplorerListResponseAttributes {
        IoCExplorerListResponseAttributes {
            data: None,
            metadata: None,
            paging: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV2::model::IoCIndicator>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV2::model::IoCExplorerListResponseMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn paging(mut self, value: crate::datadogV2::model::IoCExplorerListResponsePaging) -> Self {
        self.paging = Some(value);
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

impl Default for IoCExplorerListResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IoCExplorerListResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IoCExplorerListResponseAttributesVisitor;
        impl<'a> Visitor<'a> for IoCExplorerListResponseAttributesVisitor {
            type Value = IoCExplorerListResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::IoCIndicator>> = None;
                let mut metadata: Option<crate::datadogV2::model::IoCExplorerListResponseMetadata> =
                    None;
                let mut paging: Option<crate::datadogV2::model::IoCExplorerListResponsePaging> =
                    None;
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
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "paging" => {
                            if v.is_null() {
                                continue;
                            }
                            paging = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IoCExplorerListResponseAttributes {
                    data,
                    metadata,
                    paging,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IoCExplorerListResponseAttributesVisitor)
    }
}
