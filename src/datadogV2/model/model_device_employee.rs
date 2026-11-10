// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Directory information about the employee associated with the device,
/// sourced from a customer reference table.
/// Absent when no reference table is configured or the device has no match.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceEmployee {
    /// City where the employee is based.
    #[serde(rename = "city")]
    pub city: Option<String>,
    /// Country where the employee is based.
    #[serde(rename = "country")]
    pub country: Option<String>,
    /// Department the employee belongs to.
    #[serde(rename = "department")]
    pub department: Option<String>,
    /// Email address of the employee.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Full name of the employee.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Office the employee is assigned to.
    #[serde(rename = "office")]
    pub office: Option<String>,
    /// Team the employee belongs to.
    #[serde(rename = "team")]
    pub team: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeviceEmployee {
    pub fn new() -> DeviceEmployee {
        DeviceEmployee {
            city: None,
            country: None,
            department: None,
            email: None,
            name: None,
            office: None,
            team: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn city(mut self, value: String) -> Self {
        self.city = Some(value);
        self
    }

    pub fn country(mut self, value: String) -> Self {
        self.country = Some(value);
        self
    }

    pub fn department(mut self, value: String) -> Self {
        self.department = Some(value);
        self
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn office(mut self, value: String) -> Self {
        self.office = Some(value);
        self
    }

    pub fn team(mut self, value: String) -> Self {
        self.team = Some(value);
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

impl Default for DeviceEmployee {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeviceEmployee {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeviceEmployeeVisitor;
        impl<'a> Visitor<'a> for DeviceEmployeeVisitor {
            type Value = DeviceEmployee;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut city: Option<String> = None;
                let mut country: Option<String> = None;
                let mut department: Option<String> = None;
                let mut email: Option<String> = None;
                let mut name: Option<String> = None;
                let mut office: Option<String> = None;
                let mut team: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "city" => {
                            if v.is_null() {
                                continue;
                            }
                            city = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "country" => {
                            if v.is_null() {
                                continue;
                            }
                            country = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "department" => {
                            if v.is_null() {
                                continue;
                            }
                            department = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "office" => {
                            if v.is_null() {
                                continue;
                            }
                            office = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DeviceEmployee {
                    city,
                    country,
                    department,
                    email,
                    name,
                    office,
                    team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeviceEmployeeVisitor)
    }
}
