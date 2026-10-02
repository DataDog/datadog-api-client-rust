// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing a page of exposure SQL models.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExposureSQLModelV2DTOArray {
    /// Exposure SQL models in the current page.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTOData>,
    /// Links for navigating a paginated result set.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::ExperimentsOffsetLinks>,
    /// Pagination information for a list response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ExperimentsOffsetMeta>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsExposureSQLModelV2DTOArray {
    pub fn new(
        data: Vec<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTOData>,
    ) -> ExperimentsExposureSQLModelV2DTOArray {
        ExperimentsExposureSQLModelV2DTOArray {
            data,
            links: None,
            meta: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn links(mut self, value: crate::datadogV2::model::ExperimentsOffsetLinks) -> Self {
        self.links = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::ExperimentsOffsetMeta) -> Self {
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

impl<'de> Deserialize<'de> for ExperimentsExposureSQLModelV2DTOArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExposureSQLModelV2DTOArrayVisitor;
        impl<'a> Visitor<'a> for ExperimentsExposureSQLModelV2DTOArrayVisitor {
            type Value = ExperimentsExposureSQLModelV2DTOArray;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Vec<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTOData>,
                > = None;
                let mut links: Option<crate::datadogV2::model::ExperimentsOffsetLinks> = None;
                let mut meta: Option<crate::datadogV2::model::ExperimentsOffsetMeta> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = ExperimentsExposureSQLModelV2DTOArray {
                    data,
                    links,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExposureSQLModelV2DTOArrayVisitor)
    }
}
