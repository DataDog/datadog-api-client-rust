// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The request for a Audit Logs events list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuditLogsSearchEventsRequest {
    /// Search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::AuditLogsQueryFilter>,
    /// Global query options that are used during the query.
    /// Note: Specify either timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::AuditLogsQueryOptions>,
    /// Paging attributes for listing events.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::AuditLogsQueryPageOptions>,
    /// Sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::AuditLogsSort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuditLogsSearchEventsRequest {
    pub fn new() -> AuditLogsSearchEventsRequest {
        AuditLogsSearchEventsRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::AuditLogsQueryFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::AuditLogsQueryOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::AuditLogsQueryPageOptions) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::AuditLogsSort) -> Self {
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

impl Default for AuditLogsSearchEventsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuditLogsSearchEventsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuditLogsSearchEventsRequestVisitor;
        impl<'a> Visitor<'a> for AuditLogsSearchEventsRequestVisitor {
            type Value = AuditLogsSearchEventsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV2::model::AuditLogsQueryFilter> = None;
                let mut options: Option<crate::datadogV2::model::AuditLogsQueryOptions> = None;
                let mut page: Option<crate::datadogV2::model::AuditLogsQueryPageOptions> = None;
                let mut sort: Option<crate::datadogV2::model::AuditLogsSort> = None;
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
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::AuditLogsSort::UnparsedObject(
                                        _sort,
                                    ) => {
                                        _unparsed = true;
                                    }
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

                let content = AuditLogsSearchEventsRequest {
                    filter,
                    options,
                    page,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuditLogsSearchEventsRequestVisitor)
    }
}
