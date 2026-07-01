// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Paginated list of reference table rows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListRowsResponse {
    /// The rows.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::TableRowResourceData>,
    /// Pagination links for the list rows response.
    #[serde(rename = "links")]
    pub links: crate::datadogV2::model::ListRowsResponseLinks,
    /// Contains pagination details, including the continuation token for fetching additional rows.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ListRowsResponseMeta>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListRowsResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::TableRowResourceData>,
        links: crate::datadogV2::model::ListRowsResponseLinks,
    ) -> ListRowsResponse {
        ListRowsResponse {
            data,
            links,
            meta: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::ListRowsResponseMeta) -> Self {
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

impl<'de> Deserialize<'de> for ListRowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListRowsResponseVisitor;
        impl<'a> Visitor<'a> for ListRowsResponseVisitor {
            type Value = ListRowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::TableRowResourceData>> = None;
                let mut links: Option<crate::datadogV2::model::ListRowsResponseLinks> = None;
                let mut meta: Option<crate::datadogV2::model::ListRowsResponseMeta> = None;
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
                let links = links.ok_or_else(|| M::Error::missing_field("links"))?;

                let content = ListRowsResponse {
                    data,
                    links,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListRowsResponseVisitor)
    }
}
