// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with a list of incident integration metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentIntegrationMetadataListResponse {
    /// An array of incident integration metadata.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included:
        Option<Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseIncludedItem>>,
    /// The metadata object containing pagination metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::IncidentResponseMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentIntegrationMetadataListResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseData>,
    ) -> IncidentIntegrationMetadataListResponse {
        IncidentIntegrationMetadataListResponse {
            data,
            included: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseIncludedItem>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::IncidentResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentIntegrationMetadataListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentIntegrationMetadataListResponseVisitor;
        impl<'a> Visitor<'a> for IncidentIntegrationMetadataListResponseVisitor {
            type Value = IncidentIntegrationMetadataListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseData>,
                > = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseIncludedItem>,
                > = None;
                let mut meta: Option<crate::datadogV2::model::IncidentResponseMeta> = None;
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
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = IncidentIntegrationMetadataListResponse {
                    data,
                    included,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentIntegrationMetadataListResponseVisitor)
    }
}
