// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with Host information from Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostListResponse {
    /// Array of hosts.
    #[serde(rename = "host_list")]
    pub host_list: Option<Vec<crate::datadogV1::model::Host>>,
    /// Number of host matching the query.
    #[serde(rename = "total_matching")]
    pub total_matching: Option<i64>,
    /// Number of host returned.
    #[serde(rename = "total_returned")]
    pub total_returned: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostListResponse {
    pub fn new() -> HostListResponse {
        HostListResponse {
            host_list: None,
            total_matching: None,
            total_returned: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn host_list(mut self, value: Vec<crate::datadogV1::model::Host>) -> Self {
        self.host_list = Some(value);
        self
    }

    pub fn total_matching(mut self, value: i64) -> Self {
        self.total_matching = Some(value);
        self
    }

    pub fn total_returned(mut self, value: i64) -> Self {
        self.total_returned = Some(value);
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

impl Default for HostListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostListResponseVisitor;
        impl<'a> Visitor<'a> for HostListResponseVisitor {
            type Value = HostListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut host_list: Option<Vec<crate::datadogV1::model::Host>> = None;
                let mut total_matching: Option<i64> = None;
                let mut total_returned: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "host_list" => {
                            if v.is_null() {
                                continue;
                            }
                            host_list = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_matching" => {
                            if v.is_null() {
                                continue;
                            }
                            total_matching =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_returned" => {
                            if v.is_null() {
                                continue;
                            }
                            total_returned =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HostListResponse {
                    host_list,
                    total_matching,
                    total_returned,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostListResponseVisitor)
    }
}
