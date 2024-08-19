// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The request for a security signal list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalListRequest {
    /// Search filters for listing security signals.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SecurityMonitoringSignalListRequestFilter>,
    /// The paging attributes for listing security signals.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::SecurityMonitoringSignalListRequestPage>,
    /// The sort parameters used for querying security signals.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::SecurityMonitoringSignalsSort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalListRequest {
    pub fn new() -> SecurityMonitoringSignalListRequest {
        SecurityMonitoringSignalListRequest {
            filter: None,
            page: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalListRequestFilter,
    ) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn page(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalListRequestPage,
    ) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::SecurityMonitoringSignalsSort) -> Self {
        self.sort = Some(value);
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

impl Default for SecurityMonitoringSignalListRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalListRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalListRequestVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalListRequestVisitor {
            type Value = SecurityMonitoringSignalListRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalListRequestFilter,
                > = None;
                let mut page: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalListRequestPage,
                > = None;
                let mut sort: Option<crate::datadogV2::model::SecurityMonitoringSignalsSort> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV2::model::SecurityMonitoringSignalsSort::UnparsedObject(_sort) => {
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

                let content = SecurityMonitoringSignalListRequest {
                    filter,
                    page,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalListRequestVisitor)
    }
}
