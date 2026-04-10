// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response for listing investigations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListInvestigationsResponse {
    /// List of investigations.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::ListInvestigationsResponseData>,
    /// Pagination links for the list investigations response.
    #[serde(rename = "links")]
    pub links: crate::datadogV2::model::ListInvestigationsResponseLinks,
    /// Metadata for the list investigations response.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::ListInvestigationsResponseMeta,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListInvestigationsResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::ListInvestigationsResponseData>,
        links: crate::datadogV2::model::ListInvestigationsResponseLinks,
        meta: crate::datadogV2::model::ListInvestigationsResponseMeta,
    ) -> ListInvestigationsResponse {
        ListInvestigationsResponse {
            data,
            links,
            meta,
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

impl<'de> Deserialize<'de> for ListInvestigationsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListInvestigationsResponseVisitor;
        impl<'a> Visitor<'a> for ListInvestigationsResponseVisitor {
            type Value = ListInvestigationsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::ListInvestigationsResponseData>> =
                    None;
                let mut links: Option<crate::datadogV2::model::ListInvestigationsResponseLinks> =
                    None;
                let mut meta: Option<crate::datadogV2::model::ListInvestigationsResponseMeta> =
                    None;
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
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;

                let content = ListInvestigationsResponse {
                    data,
                    links,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListInvestigationsResponseVisitor)
    }
}
