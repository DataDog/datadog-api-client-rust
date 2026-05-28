// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Common health and identity attributes shared by every End User Device Monitoring device record.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceBaseDataAttributes {
    /// Public key of the Datadog Agent installed on the device.
    #[serde(rename = "agent_key")]
    pub agent_key: Option<String>,
    /// Version of the Datadog Agent installed on the device.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// Maximum battery capacity expressed as a percentage of the device's design capacity.
    #[serde(rename = "battery_max_capacity_pct")]
    pub battery_max_capacity_pct: Option<i64>,
    /// Number of physical CPU cores on the device.
    #[serde(rename = "cpu_cores")]
    pub cpu_cores: Option<i64>,
    /// Number of logical CPU processors (hardware threads) on the device.
    #[serde(rename = "cpu_logical_processors")]
    pub cpu_logical_processors: Option<i64>,
    /// Human-readable name of the device's CPU model.
    #[serde(rename = "cpu_model")]
    pub cpu_model: Option<String>,
    /// Average CPU usage on the device, as a percentage between 0 and 100.
    #[serde(rename = "cpu_usage")]
    pub cpu_usage: Option<f64>,
    /// Average disk usage on the device, as a percentage between 0 and 100.
    #[serde(rename = "disk_usage")]
    pub disk_usage: Option<f64>,
    /// Last observed IPv4 or IPv6 address of the device.
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    /// List of issue identifiers currently affecting the device.
    /// References entries returned by the issues endpoint.
    #[serde(rename = "issues")]
    pub issues: Option<Vec<String>>,
    /// Name of the operating system kernel running on the device.
    #[serde(rename = "kernel_name")]
    pub kernel_name: Option<String>,
    /// Timestamp of the most recent telemetry received from the device, in RFC 3339 format.
    #[serde(rename = "last_seen")]
    pub last_seen: Option<String>,
    /// Manufacturer of the device.
    #[serde(rename = "manufacturer")]
    pub manufacturer: Option<String>,
    /// Average memory usage on the device, as a percentage between 0 and 100.
    #[serde(rename = "mem_usage")]
    pub mem_usage: Option<f64>,
    /// Total amount of physical memory available on the device, in kilobytes.
    #[serde(rename = "memory_total_kb")]
    pub memory_total_kb: Option<i64>,
    /// Marketing or product name of the device model.
    #[serde(rename = "model_name")]
    pub model_name: Option<String>,
    /// Manufacturer-assigned model number of the device.
    #[serde(rename = "model_number")]
    pub model_number: Option<String>,
    /// Operating system family running on the device (for example, `mac`, `windows`, or `linux`).
    #[serde(rename = "os")]
    pub os: Option<String>,
    /// Operating system version running on the device.
    #[serde(rename = "os_version")]
    pub os_version: Option<String>,
    /// Datadog resource identifier for the device.
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    /// Serial number assigned to the device by its manufacturer.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    /// Health status of the device computed from its issues and recent telemetry.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Hardware type of the device (for example, `laptop`, `desktop`, or `mobile`).
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Time elapsed since the device last booted, in seconds.
    #[serde(rename = "uptime")]
    pub uptime: Option<f64>,
    /// BSSID (MAC address of the access point) of the wireless network the device is
    /// currently connected to.
    #[serde(rename = "wlan_bssid")]
    pub wlan_bssid: Option<String>,
    /// Received signal strength indicator of the device's current wireless connection, in dBm.
    #[serde(rename = "wlan_rssi")]
    pub wlan_rssi: Option<f64>,
    /// SSID of the wireless network the device is currently connected to.
    #[serde(rename = "wlan_ssid")]
    pub wlan_ssid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeviceBaseDataAttributes {
    pub fn new() -> DeviceBaseDataAttributes {
        DeviceBaseDataAttributes {
            agent_key: None,
            agent_version: None,
            battery_max_capacity_pct: None,
            cpu_cores: None,
            cpu_logical_processors: None,
            cpu_model: None,
            cpu_usage: None,
            disk_usage: None,
            ip_address: None,
            issues: None,
            kernel_name: None,
            last_seen: None,
            manufacturer: None,
            mem_usage: None,
            memory_total_kb: None,
            model_name: None,
            model_number: None,
            os: None,
            os_version: None,
            resource_id: None,
            serial_number: None,
            status: None,
            type_: None,
            uptime: None,
            wlan_bssid: None,
            wlan_rssi: None,
            wlan_ssid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_key(mut self, value: String) -> Self {
        self.agent_key = Some(value);
        self
    }

    pub fn agent_version(mut self, value: String) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn battery_max_capacity_pct(mut self, value: i64) -> Self {
        self.battery_max_capacity_pct = Some(value);
        self
    }

    pub fn cpu_cores(mut self, value: i64) -> Self {
        self.cpu_cores = Some(value);
        self
    }

    pub fn cpu_logical_processors(mut self, value: i64) -> Self {
        self.cpu_logical_processors = Some(value);
        self
    }

    pub fn cpu_model(mut self, value: String) -> Self {
        self.cpu_model = Some(value);
        self
    }

    pub fn cpu_usage(mut self, value: f64) -> Self {
        self.cpu_usage = Some(value);
        self
    }

    pub fn disk_usage(mut self, value: f64) -> Self {
        self.disk_usage = Some(value);
        self
    }

    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    pub fn issues(mut self, value: Vec<String>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn kernel_name(mut self, value: String) -> Self {
        self.kernel_name = Some(value);
        self
    }

    pub fn last_seen(mut self, value: String) -> Self {
        self.last_seen = Some(value);
        self
    }

    pub fn manufacturer(mut self, value: String) -> Self {
        self.manufacturer = Some(value);
        self
    }

    pub fn mem_usage(mut self, value: f64) -> Self {
        self.mem_usage = Some(value);
        self
    }

    pub fn memory_total_kb(mut self, value: i64) -> Self {
        self.memory_total_kb = Some(value);
        self
    }

    pub fn model_name(mut self, value: String) -> Self {
        self.model_name = Some(value);
        self
    }

    pub fn model_number(mut self, value: String) -> Self {
        self.model_number = Some(value);
        self
    }

    pub fn os(mut self, value: String) -> Self {
        self.os = Some(value);
        self
    }

    pub fn os_version(mut self, value: String) -> Self {
        self.os_version = Some(value);
        self
    }

    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
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

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn uptime(mut self, value: f64) -> Self {
        self.uptime = Some(value);
        self
    }

    pub fn wlan_bssid(mut self, value: String) -> Self {
        self.wlan_bssid = Some(value);
        self
    }

    pub fn wlan_rssi(mut self, value: f64) -> Self {
        self.wlan_rssi = Some(value);
        self
    }

    pub fn wlan_ssid(mut self, value: String) -> Self {
        self.wlan_ssid = Some(value);
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

impl Default for DeviceBaseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeviceBaseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeviceBaseDataAttributesVisitor;
        impl<'a> Visitor<'a> for DeviceBaseDataAttributesVisitor {
            type Value = DeviceBaseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_key: Option<String> = None;
                let mut agent_version: Option<String> = None;
                let mut battery_max_capacity_pct: Option<i64> = None;
                let mut cpu_cores: Option<i64> = None;
                let mut cpu_logical_processors: Option<i64> = None;
                let mut cpu_model: Option<String> = None;
                let mut cpu_usage: Option<f64> = None;
                let mut disk_usage: Option<f64> = None;
                let mut ip_address: Option<String> = None;
                let mut issues: Option<Vec<String>> = None;
                let mut kernel_name: Option<String> = None;
                let mut last_seen: Option<String> = None;
                let mut manufacturer: Option<String> = None;
                let mut mem_usage: Option<f64> = None;
                let mut memory_total_kb: Option<i64> = None;
                let mut model_name: Option<String> = None;
                let mut model_number: Option<String> = None;
                let mut os: Option<String> = None;
                let mut os_version: Option<String> = None;
                let mut resource_id: Option<String> = None;
                let mut serial_number: Option<String> = None;
                let mut status: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut uptime: Option<f64> = None;
                let mut wlan_bssid: Option<String> = None;
                let mut wlan_rssi: Option<f64> = None;
                let mut wlan_ssid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_key" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_version" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "battery_max_capacity_pct" => {
                            if v.is_null() {
                                continue;
                            }
                            battery_max_capacity_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cpu_cores" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu_cores = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cpu_logical_processors" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu_logical_processors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cpu_model" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu_model = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cpu_usage" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cpu_usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disk_usage" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            disk_usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ip_address" => {
                            if v.is_null() {
                                continue;
                            }
                            ip_address = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issues" => {
                            if v.is_null() {
                                continue;
                            }
                            issues = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kernel_name" => {
                            if v.is_null() {
                                continue;
                            }
                            kernel_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen" => {
                            if v.is_null() {
                                continue;
                            }
                            last_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "manufacturer" => {
                            if v.is_null() {
                                continue;
                            }
                            manufacturer =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mem_usage" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            mem_usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "memory_total_kb" => {
                            if v.is_null() {
                                continue;
                            }
                            memory_total_kb =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_name" => {
                            if v.is_null() {
                                continue;
                            }
                            model_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_number" => {
                            if v.is_null() {
                                continue;
                            }
                            model_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os" => {
                            if v.is_null() {
                                continue;
                            }
                            os = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_version" => {
                            if v.is_null() {
                                continue;
                            }
                            os_version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_id =
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
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uptime" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            uptime = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "wlan_bssid" => {
                            if v.is_null() {
                                continue;
                            }
                            wlan_bssid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "wlan_rssi" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            wlan_rssi = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "wlan_ssid" => {
                            if v.is_null() {
                                continue;
                            }
                            wlan_ssid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DeviceBaseDataAttributes {
                    agent_key,
                    agent_version,
                    battery_max_capacity_pct,
                    cpu_cores,
                    cpu_logical_processors,
                    cpu_model,
                    cpu_usage,
                    disk_usage,
                    ip_address,
                    issues,
                    kernel_name,
                    last_seen,
                    manufacturer,
                    mem_usage,
                    memory_total_kb,
                    model_name,
                    model_number,
                    os,
                    os_version,
                    resource_id,
                    serial_number,
                    status,
                    type_,
                    uptime,
                    wlan_bssid,
                    wlan_rssi,
                    wlan_ssid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeviceBaseDataAttributesVisitor)
    }
}
