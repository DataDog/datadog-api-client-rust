// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Meta attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalsListResponseMeta {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::SecurityMonitoringSignalsListResponseMetaPage>,
    /// The unique identifier of the request.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The status of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalsListResponseMeta {
    pub fn new() -> SecurityMonitoringSignalsListResponseMeta {
        SecurityMonitoringSignalsListResponseMeta {
            elapsed: None,
            page: None,
            request_id: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn elapsed(mut self, value: i64) -> Self {
        self.elapsed = Some(value);
        self
    }

    pub fn page(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalsListResponseMetaPage,
    ) -> Self {
        self.page = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
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

impl Default for SecurityMonitoringSignalsListResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalsListResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalsListResponseMetaVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalsListResponseMetaVisitor {
            type Value = SecurityMonitoringSignalsListResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed: Option<i64> = None;
                let mut page: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalsListResponseMetaPage,
                > = None;
                let mut request_id: Option<String> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elapsed" => {
                            if v.is_null() {
                                continue;
                            }
                            elapsed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringSignalsListResponseMeta {
                    elapsed,
                    page,
                    request_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalsListResponseMetaVisitor)
    }
}
