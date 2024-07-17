// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The device attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetDeviceAttributes {
    /// A description of the device.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The type of the device.
    #[serde(rename = "device_type")]
    pub device_type: Option<String>,
    /// The integration of the device.
    #[serde(rename = "integration")]
    pub integration: Option<String>,
    /// The IP address of the device.
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    /// The location of the device.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// The model of the device.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// The name of the device.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The operating system hostname of the device.
    #[serde(rename = "os_hostname")]
    pub os_hostname: Option<String>,
    /// The operating system name of the device.
    #[serde(rename = "os_name")]
    pub os_name: Option<String>,
    /// The operating system version of the device.
    #[serde(rename = "os_version")]
    pub os_version: Option<String>,
    /// The ping status of the device.
    #[serde(rename = "ping_status")]
    pub ping_status: Option<String>,
    /// The product name of the device.
    #[serde(rename = "product_name")]
    pub product_name: Option<String>,
    /// The serial number of the device.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    /// The status of the device.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The subnet of the device.
    #[serde(rename = "subnet")]
    pub subnet: Option<String>,
    /// The device `sys_object_id`.
    #[serde(rename = "sys_object_id")]
    pub sys_object_id: Option<String>,
    /// A list of tags associated with the device.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The vendor of the device.
    #[serde(rename = "vendor")]
    pub vendor: Option<String>,
    /// The version of the device.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetDeviceAttributes {
    pub fn new() -> GetDeviceAttributes {
        GetDeviceAttributes {
            description: None,
            device_type: None,
            integration: None,
            ip_address: None,
            location: None,
            model: None,
            name: None,
            os_hostname: None,
            os_name: None,
            os_version: None,
            ping_status: None,
            product_name: None,
            serial_number: None,
            status: None,
            subnet: None,
            sys_object_id: None,
            tags: None,
            vendor: None,
            version: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn device_type(mut self, value: String) -> Self {
        self.device_type = Some(value);
        self
    }

    pub fn integration(mut self, value: String) -> Self {
        self.integration = Some(value);
        self
    }

    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }

    pub fn model(mut self, value: String) -> Self {
        self.model = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn os_hostname(mut self, value: String) -> Self {
        self.os_hostname = Some(value);
        self
    }

    pub fn os_name(mut self, value: String) -> Self {
        self.os_name = Some(value);
        self
    }

    pub fn os_version(mut self, value: String) -> Self {
        self.os_version = Some(value);
        self
    }

    pub fn ping_status(mut self, value: String) -> Self {
        self.ping_status = Some(value);
        self
    }

    pub fn product_name(mut self, value: String) -> Self {
        self.product_name = Some(value);
        self
    }

    pub fn serial_number(mut self, value: String) -> Self {
        self.serial_number = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subnet(mut self, value: String) -> Self {
        self.subnet = Some(value);
        self
    }

    pub fn sys_object_id(mut self, value: String) -> Self {
        self.sys_object_id = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn vendor(mut self, value: String) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for GetDeviceAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetDeviceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetDeviceAttributesVisitor;
        impl<'a> Visitor<'a> for GetDeviceAttributesVisitor {
            type Value = GetDeviceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut device_type: Option<String> = None;
                let mut integration: Option<String> = None;
                let mut ip_address: Option<String> = None;
                let mut location: Option<String> = None;
                let mut model: Option<String> = None;
                let mut name: Option<String> = None;
                let mut os_hostname: Option<String> = None;
                let mut os_name: Option<String> = None;
                let mut os_version: Option<String> = None;
                let mut ping_status: Option<String> = None;
                let mut product_name: Option<String> = None;
                let mut serial_number: Option<String> = None;
                let mut status: Option<String> = None;
                let mut subnet: Option<String> = None;
                let mut sys_object_id: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut vendor: Option<String> = None;
                let mut version: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_type" => {
                            if v.is_null() {
                                continue;
                            }
                            device_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            if v.is_null() {
                                continue;
                            }
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ip_address" => {
                            if v.is_null() {
                                continue;
                            }
                            ip_address = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model" => {
                            if v.is_null() {
                                continue;
                            }
                            model = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            os_hostname =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_name" => {
                            if v.is_null() {
                                continue;
                            }
                            os_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_version" => {
                            if v.is_null() {
                                continue;
                            }
                            os_version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ping_status" => {
                            if v.is_null() {
                                continue;
                            }
                            ping_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_name" => {
                            if v.is_null() {
                                continue;
                            }
                            product_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serial_number" => {
                            if v.is_null() {
                                continue;
                            }
                            serial_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subnet" => {
                            if v.is_null() {
                                continue;
                            }
                            subnet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sys_object_id" => {
                            if v.is_null() {
                                continue;
                            }
                            sys_object_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vendor" => {
                            if v.is_null() {
                                continue;
                            }
                            vendor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = GetDeviceAttributes {
                    description,
                    device_type,
                    integration,
                    ip_address,
                    location,
                    model,
                    name,
                    os_hostname,
                    os_name,
                    os_version,
                    ping_status,
                    product_name,
                    serial_number,
                    status,
                    subnet,
                    sys_object_id,
                    tags,
                    vendor,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetDeviceAttributesVisitor)
    }
}
