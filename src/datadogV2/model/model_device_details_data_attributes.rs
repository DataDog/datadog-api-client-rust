// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Extended set of attributes for a single End User Device Monitoring device,
/// including Cloud Network Monitoring and battery metrics.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceDetailsDataAttributes {
    /// Public key of the Datadog Agent installed on the device.
    #[serde(rename = "agent_key")]
    pub agent_key: Option<String>,
    /// Version of the Datadog Agent installed on the device.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// Current battery charge level as a percentage between 0 and 100.
    #[serde(rename = "battery_charge_pct")]
    pub battery_charge_pct: Option<i64>,
    /// Rate at which the battery is charging or discharging, in milliamperes.
    /// Negative values indicate discharge.
    #[serde(rename = "battery_charge_rate")]
    pub battery_charge_rate: Option<i64>,
    /// Whether the device's battery health has degraded to a critical level.
    #[serde(rename = "battery_critical")]
    pub battery_critical: Option<bool>,
    /// Number of full charge cycles the battery has gone through.
    #[serde(rename = "battery_cycle_count")]
    pub battery_cycle_count: Option<i64>,
    /// Maximum battery capacity expressed as a percentage of the device's design capacity.
    #[serde(rename = "battery_max_capacity_pct")]
    pub battery_max_capacity_pct: Option<i64>,
    /// Whether Cloud Network Monitoring is enabled on the device.
    #[serde(rename = "cnm_enabled")]
    pub cnm_enabled: Option<bool>,
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
    /// Directory information about the employee associated with the device,
    /// sourced from a customer reference table.
    /// Absent when no reference table is configured or the device has no match.
    #[serde(rename = "employee")]
    pub employee: Option<crate::datadogV2::model::DeviceEmployee>,
    /// Location derived from the device's public IP address.
    /// Absent when no location data is available.
    #[serde(rename = "geo_ip")]
    pub geo_ip: Option<crate::datadogV2::model::DeviceGeoIP>,
    /// Last observed IPv4 or IPv6 address of the device.
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    /// List of issue identifiers currently affecting the device.
    #[serde(rename = "issues")]
    pub issues: Option<Vec<String>>,
    /// Name of the operating system kernel running on the device.
    #[serde(rename = "kernel_name")]
    pub kernel_name: Option<String>,
    /// Timestamp of the most recent device reboot, in RFC 3339 format.
    #[serde(rename = "last_reboot")]
    pub last_reboot: Option<String>,
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
    /// Operating system family running on the device, as reported by the Datadog Agent
    /// (for example, `macOS`, `Windows`, or `linux`). Empty when unknown.
    #[serde(rename = "os")]
    pub os: Option<String>,
    /// Operating system version running on the device.
    #[serde(rename = "os_version")]
    pub os_version: Option<String>,
    /// Number of TCP packets sent by the device over the query window,
    /// as reported by Cloud Network Monitoring.
    #[serde(rename = "packet_sent")]
    pub packet_sent: Option<f64>,
    /// Datadog resource identifier for the device.
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    /// Number of TCP packets retransmitted by the device over the query window.
    #[serde(rename = "retransmit")]
    pub retransmit: Option<f64>,
    /// Percentage of sent TCP packets that were retransmitted, between 0 and 100.
    #[serde(rename = "retransmit_pct")]
    pub retransmit_pct: Option<f64>,
    /// Serial number assigned to the device by its manufacturer.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    /// Health status of the device, computed from its issues and recent telemetry.
    /// `inactive` means the device stopped reporting. `unknown` means no telemetry is available yet.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::DeviceStatus>,
    /// Number of TCP connection attempts canceled by the device over the query window.
    #[serde(rename = "tcp_canceled")]
    pub tcp_canceled: Option<f64>,
    /// Number of TCP connections established by the device over the query window.
    #[serde(rename = "tcp_established")]
    pub tcp_established: Option<f64>,
    /// Percentage of TCP connection attempts that failed, between 0 and 100.
    #[serde(rename = "tcp_establishment_failure_pct")]
    pub tcp_establishment_failure_pct: Option<f64>,
    /// Number of TCP connection attempts that failed over the query window.
    #[serde(rename = "tcp_establishment_failures")]
    pub tcp_establishment_failures: Option<f64>,
    /// Number of TCP connection attempts that failed because the host was unreachable.
    #[serde(rename = "tcp_host_unreachable")]
    pub tcp_host_unreachable: Option<f64>,
    /// Number of TCP connection attempts that failed because the network was unreachable.
    #[serde(rename = "tcp_network_unreachable")]
    pub tcp_network_unreachable: Option<f64>,
    /// Number of TCP connection attempts refused by the remote host over the query window.
    #[serde(rename = "tcp_refusals")]
    pub tcp_refusals: Option<f64>,
    /// Number of TCP connection attempts that timed out over the query window.
    #[serde(rename = "tcp_timeouts")]
    pub tcp_timeouts: Option<f64>,
    /// Hardware type of the device (for example, `laptop` or `desktop`). `unknown` when the type cannot be determined.
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

impl DeviceDetailsDataAttributes {
    pub fn new() -> DeviceDetailsDataAttributes {
        DeviceDetailsDataAttributes {
            agent_key: None,
            agent_version: None,
            battery_charge_pct: None,
            battery_charge_rate: None,
            battery_critical: None,
            battery_cycle_count: None,
            battery_max_capacity_pct: None,
            cnm_enabled: None,
            cpu_cores: None,
            cpu_logical_processors: None,
            cpu_model: None,
            cpu_usage: None,
            disk_usage: None,
            employee: None,
            geo_ip: None,
            ip_address: None,
            issues: None,
            kernel_name: None,
            last_reboot: None,
            last_seen: None,
            manufacturer: None,
            mem_usage: None,
            memory_total_kb: None,
            model_name: None,
            model_number: None,
            os: None,
            os_version: None,
            packet_sent: None,
            resource_id: None,
            retransmit: None,
            retransmit_pct: None,
            serial_number: None,
            status: None,
            tcp_canceled: None,
            tcp_established: None,
            tcp_establishment_failure_pct: None,
            tcp_establishment_failures: None,
            tcp_host_unreachable: None,
            tcp_network_unreachable: None,
            tcp_refusals: None,
            tcp_timeouts: None,
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

    pub fn battery_charge_pct(mut self, value: i64) -> Self {
        self.battery_charge_pct = Some(value);
        self
    }

    pub fn battery_charge_rate(mut self, value: i64) -> Self {
        self.battery_charge_rate = Some(value);
        self
    }

    pub fn battery_critical(mut self, value: bool) -> Self {
        self.battery_critical = Some(value);
        self
    }

    pub fn battery_cycle_count(mut self, value: i64) -> Self {
        self.battery_cycle_count = Some(value);
        self
    }

    pub fn battery_max_capacity_pct(mut self, value: i64) -> Self {
        self.battery_max_capacity_pct = Some(value);
        self
    }

    pub fn cnm_enabled(mut self, value: bool) -> Self {
        self.cnm_enabled = Some(value);
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

    pub fn employee(mut self, value: crate::datadogV2::model::DeviceEmployee) -> Self {
        self.employee = Some(value);
        self
    }

    pub fn geo_ip(mut self, value: crate::datadogV2::model::DeviceGeoIP) -> Self {
        self.geo_ip = Some(value);
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

    pub fn last_reboot(mut self, value: String) -> Self {
        self.last_reboot = Some(value);
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

    pub fn packet_sent(mut self, value: f64) -> Self {
        self.packet_sent = Some(value);
        self
    }

    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    pub fn retransmit(mut self, value: f64) -> Self {
        self.retransmit = Some(value);
        self
    }

    pub fn retransmit_pct(mut self, value: f64) -> Self {
        self.retransmit_pct = Some(value);
        self
    }

    pub fn serial_number(mut self, value: String) -> Self {
        self.serial_number = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::DeviceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tcp_canceled(mut self, value: f64) -> Self {
        self.tcp_canceled = Some(value);
        self
    }

    pub fn tcp_established(mut self, value: f64) -> Self {
        self.tcp_established = Some(value);
        self
    }

    pub fn tcp_establishment_failure_pct(mut self, value: f64) -> Self {
        self.tcp_establishment_failure_pct = Some(value);
        self
    }

    pub fn tcp_establishment_failures(mut self, value: f64) -> Self {
        self.tcp_establishment_failures = Some(value);
        self
    }

    pub fn tcp_host_unreachable(mut self, value: f64) -> Self {
        self.tcp_host_unreachable = Some(value);
        self
    }

    pub fn tcp_network_unreachable(mut self, value: f64) -> Self {
        self.tcp_network_unreachable = Some(value);
        self
    }

    pub fn tcp_refusals(mut self, value: f64) -> Self {
        self.tcp_refusals = Some(value);
        self
    }

    pub fn tcp_timeouts(mut self, value: f64) -> Self {
        self.tcp_timeouts = Some(value);
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

impl Default for DeviceDetailsDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeviceDetailsDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeviceDetailsDataAttributesVisitor;
        impl<'a> Visitor<'a> for DeviceDetailsDataAttributesVisitor {
            type Value = DeviceDetailsDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_key: Option<String> = None;
                let mut agent_version: Option<String> = None;
                let mut battery_charge_pct: Option<i64> = None;
                let mut battery_charge_rate: Option<i64> = None;
                let mut battery_critical: Option<bool> = None;
                let mut battery_cycle_count: Option<i64> = None;
                let mut battery_max_capacity_pct: Option<i64> = None;
                let mut cnm_enabled: Option<bool> = None;
                let mut cpu_cores: Option<i64> = None;
                let mut cpu_logical_processors: Option<i64> = None;
                let mut cpu_model: Option<String> = None;
                let mut cpu_usage: Option<f64> = None;
                let mut disk_usage: Option<f64> = None;
                let mut employee: Option<crate::datadogV2::model::DeviceEmployee> = None;
                let mut geo_ip: Option<crate::datadogV2::model::DeviceGeoIP> = None;
                let mut ip_address: Option<String> = None;
                let mut issues: Option<Vec<String>> = None;
                let mut kernel_name: Option<String> = None;
                let mut last_reboot: Option<String> = None;
                let mut last_seen: Option<String> = None;
                let mut manufacturer: Option<String> = None;
                let mut mem_usage: Option<f64> = None;
                let mut memory_total_kb: Option<i64> = None;
                let mut model_name: Option<String> = None;
                let mut model_number: Option<String> = None;
                let mut os: Option<String> = None;
                let mut os_version: Option<String> = None;
                let mut packet_sent: Option<f64> = None;
                let mut resource_id: Option<String> = None;
                let mut retransmit: Option<f64> = None;
                let mut retransmit_pct: Option<f64> = None;
                let mut serial_number: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::DeviceStatus> = None;
                let mut tcp_canceled: Option<f64> = None;
                let mut tcp_established: Option<f64> = None;
                let mut tcp_establishment_failure_pct: Option<f64> = None;
                let mut tcp_establishment_failures: Option<f64> = None;
                let mut tcp_host_unreachable: Option<f64> = None;
                let mut tcp_network_unreachable: Option<f64> = None;
                let mut tcp_refusals: Option<f64> = None;
                let mut tcp_timeouts: Option<f64> = None;
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
                        "battery_charge_pct" => {
                            if v.is_null() {
                                continue;
                            }
                            battery_charge_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "battery_charge_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            battery_charge_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "battery_critical" => {
                            if v.is_null() {
                                continue;
                            }
                            battery_critical =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "battery_cycle_count" => {
                            if v.is_null() {
                                continue;
                            }
                            battery_cycle_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "battery_max_capacity_pct" => {
                            if v.is_null() {
                                continue;
                            }
                            battery_max_capacity_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cnm_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            cnm_enabled =
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
                        "employee" => {
                            if v.is_null() {
                                continue;
                            }
                            employee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "geo_ip" => {
                            if v.is_null() {
                                continue;
                            }
                            geo_ip = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "last_reboot" => {
                            if v.is_null() {
                                continue;
                            }
                            last_reboot =
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
                        "packet_sent" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            packet_sent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retransmit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            retransmit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retransmit_pct" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            retransmit_pct =
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
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::DeviceStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tcp_canceled" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_canceled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_established" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_established =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_establishment_failure_pct" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_establishment_failure_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_establishment_failures" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_establishment_failures =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_host_unreachable" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_host_unreachable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_network_unreachable" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_network_unreachable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_refusals" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_refusals =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_timeouts" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tcp_timeouts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = DeviceDetailsDataAttributes {
                    agent_key,
                    agent_version,
                    battery_charge_pct,
                    battery_charge_rate,
                    battery_critical,
                    battery_cycle_count,
                    battery_max_capacity_pct,
                    cnm_enabled,
                    cpu_cores,
                    cpu_logical_processors,
                    cpu_model,
                    cpu_usage,
                    disk_usage,
                    employee,
                    geo_ip,
                    ip_address,
                    issues,
                    kernel_name,
                    last_reboot,
                    last_seen,
                    manufacturer,
                    mem_usage,
                    memory_total_kb,
                    model_name,
                    model_number,
                    os,
                    os_version,
                    packet_sent,
                    resource_id,
                    retransmit,
                    retransmit_pct,
                    serial_number,
                    status,
                    tcp_canceled,
                    tcp_established,
                    tcp_establishment_failure_pct,
                    tcp_establishment_failures,
                    tcp_host_unreachable,
                    tcp_network_unreachable,
                    tcp_refusals,
                    tcp_timeouts,
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

        deserializer.deserialize_any(DeviceDetailsDataAttributesVisitor)
    }
}
