// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing a list of report schedules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReportScheduleListResponse {
    /// The list of report schedules.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::ReportScheduleListResponseData>,
    /// Related resources included with the report schedules, such as authors and rendered resources.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::ReportScheduleIncludedResource>>,
    /// Pagination links for navigating a report schedule list response.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::ReportScheduleListResponseLinks>,
    /// Metadata for a paginated report schedule list response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ReportScheduleListResponseMeta>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReportScheduleListResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::ReportScheduleListResponseData>,
    ) -> ReportScheduleListResponse {
        ReportScheduleListResponse {
            data,
            included: None,
            links: None,
            meta: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::ReportScheduleIncludedResource>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn links(
        mut self,
        value: crate::datadogV2::model::ReportScheduleListResponseLinks,
    ) -> Self {
        self.links = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::ReportScheduleListResponseMeta) -> Self {
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

impl<'de> Deserialize<'de> for ReportScheduleListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReportScheduleListResponseVisitor;
        impl<'a> Visitor<'a> for ReportScheduleListResponseVisitor {
            type Value = ReportScheduleListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::ReportScheduleListResponseData>> =
                    None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::ReportScheduleIncludedResource>,
                > = None;
                let mut links: Option<crate::datadogV2::model::ReportScheduleListResponseLinks> =
                    None;
                let mut meta: Option<crate::datadogV2::model::ReportScheduleListResponseMeta> =
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
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ReportScheduleListResponse {
                    data,
                    included,
                    links,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReportScheduleListResponseVisitor)
    }
}
