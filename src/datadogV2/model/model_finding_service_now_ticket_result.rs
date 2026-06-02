// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result of the ServiceNow ticket creation or attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FindingServiceNowTicketResult {
    /// ServiceNow instance name extracted from the ticket URL.
    #[serde(rename = "instance_name")]
    pub instance_name: Option<String>,
    /// Unique identifier of the ServiceNow incident record.
    #[serde(rename = "sys_id")]
    pub sys_id: Option<String>,
    /// Direct link to the ServiceNow incident record.
    #[serde(rename = "sys_target_link")]
    pub sys_target_link: Option<String>,
    /// Unique identifier of the target ServiceNow record.
    #[serde(rename = "sys_target_sys_id")]
    pub sys_target_sys_id: Option<String>,
    /// ServiceNow table containing the incident record.
    #[serde(rename = "table_name")]
    pub table_name: Option<String>,
    /// URL of the ServiceNow incident record.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FindingServiceNowTicketResult {
    pub fn new() -> FindingServiceNowTicketResult {
        FindingServiceNowTicketResult {
            instance_name: None,
            sys_id: None,
            sys_target_link: None,
            sys_target_sys_id: None,
            table_name: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn instance_name(mut self, value: String) -> Self {
        self.instance_name = Some(value);
        self
    }

    pub fn sys_id(mut self, value: String) -> Self {
        self.sys_id = Some(value);
        self
    }

    pub fn sys_target_link(mut self, value: String) -> Self {
        self.sys_target_link = Some(value);
        self
    }

    pub fn sys_target_sys_id(mut self, value: String) -> Self {
        self.sys_target_sys_id = Some(value);
        self
    }

    pub fn table_name(mut self, value: String) -> Self {
        self.table_name = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for FindingServiceNowTicketResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FindingServiceNowTicketResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FindingServiceNowTicketResultVisitor;
        impl<'a> Visitor<'a> for FindingServiceNowTicketResultVisitor {
            type Value = FindingServiceNowTicketResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut instance_name: Option<String> = None;
                let mut sys_id: Option<String> = None;
                let mut sys_target_link: Option<String> = None;
                let mut sys_target_sys_id: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "instance_name" => {
                            if v.is_null() {
                                continue;
                            }
                            instance_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sys_id" => {
                            if v.is_null() {
                                continue;
                            }
                            sys_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sys_target_link" => {
                            if v.is_null() {
                                continue;
                            }
                            sys_target_link =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sys_target_sys_id" => {
                            if v.is_null() {
                                continue;
                            }
                            sys_target_sys_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            if v.is_null() {
                                continue;
                            }
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FindingServiceNowTicketResult {
                    instance_name,
                    sys_id,
                    sys_target_link,
                    sys_target_sys_id,
                    table_name,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FindingServiceNowTicketResultVisitor)
    }
}
