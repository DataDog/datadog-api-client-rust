// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Detailed attributes of a network health insight.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NetworkHealthInsightAttributes {
    /// AWS account identifier where the certificate is located. Only set for `tls-cert` insights.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// ARN or identifier of the certificate. Only set for `tls-cert` insights.
    #[serde(rename = "certificate_id")]
    pub certificate_id: Option<String>,
    /// Percentage of the certificate's validity period that has elapsed, ranging from 0 to 100.
    /// Only set for `tls-cert` insights.
    #[serde(rename = "certificate_lifetime_percent")]
    pub certificate_lifetime_percent: Option<f64>,
    /// AWS region where the client is located. Only set for `tls-cert` insights.
    #[serde(rename = "client_region")]
    pub client_region: Option<String>,
    /// Name of the service making the request (DNS query or TLS-secured connection).
    /// Set to `N/A` when the client service cannot be determined.
    #[serde(rename = "client_service")]
    pub client_service: Option<String>,
    /// Number of days remaining until the certificate expires. Negative values indicate the
    /// certificate has already expired. Only set for `tls-cert` insights.
    #[serde(rename = "days_until_expiration")]
    pub days_until_expiration: Option<i64>,
    /// Domain name that was being resolved when the DNS failure occurred. Only set for `dns` insights.
    #[serde(rename = "dns_query")]
    pub dns_query: Option<String>,
    /// DNS server that received the failing query. Only set for `dns` insights.
    #[serde(rename = "dns_server")]
    pub dns_server: Option<String>,
    /// Domain name covered by the certificate. Only set for `tls-cert` insights.
    #[serde(rename = "domain_name")]
    pub domain_name: Option<String>,
    /// Count of failed events observed during the query window. Only set for `dns`, `tcp`,
    /// and `security-group` insights.
    #[serde(rename = "failure_magnitude")]
    pub failure_magnitude: Option<i64>,
    /// Percentage of requests that failed during the query window, ranging from 0 to 100.
    /// Only set for `dns`, `tcp`, and `security-group` insights.
    #[serde(rename = "failure_rate")]
    pub failure_rate: Option<f64>,
    /// Specific failure type within the insight category. For DNS insights: `timeout`, `nxdomain`,
    /// `servfail`, or `general_failure`. For TLS certificate insights: `expired` or `expiring_soon`.
    /// For security group insights: `denied`.
    #[serde(rename = "failure_type")]
    pub failure_type: Option<crate::datadogV2::model::NetworkHealthInsightFailureType>,
    /// ARN of the load balancer using the certificate. Only set for `tls-cert` insights.
    #[serde(rename = "loadbalancer_id")]
    pub loadbalancer_id: Option<String>,
    /// AWS region where the server or load balancer is located. Only set for `tls-cert` insights.
    #[serde(rename = "server_region")]
    pub server_region: Option<String>,
    /// Name of the target service the client was trying to reach.
    #[serde(rename = "server_service")]
    pub server_service: Option<String>,
    /// Total number of requests observed during the query window. Provides context for
    /// `failure_magnitude` and `failure_rate`. Only set for `dns`, `tcp`, and `security-group` insights.
    #[serde(rename = "total_requests")]
    pub total_requests: Option<i64>,
    /// Network traffic volume metrics between the client and server services during the query window.
    #[serde(rename = "traffic_volume")]
    pub traffic_volume: Option<crate::datadogV2::model::NetworkHealthInsightTrafficVolume>,
    /// Category of network health insight. Indicates whether the insight relates to a DNS issue (`dns`),
    /// a TCP issue (`tcp`), a TLS certificate issue (`tls-cert`), or a security group denial (`security-group`).
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::NetworkHealthInsightCategory>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NetworkHealthInsightAttributes {
    pub fn new() -> NetworkHealthInsightAttributes {
        NetworkHealthInsightAttributes {
            account_id: None,
            certificate_id: None,
            certificate_lifetime_percent: None,
            client_region: None,
            client_service: None,
            days_until_expiration: None,
            dns_query: None,
            dns_server: None,
            domain_name: None,
            failure_magnitude: None,
            failure_rate: None,
            failure_type: None,
            loadbalancer_id: None,
            server_region: None,
            server_service: None,
            total_requests: None,
            traffic_volume: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn certificate_id(mut self, value: String) -> Self {
        self.certificate_id = Some(value);
        self
    }

    pub fn certificate_lifetime_percent(mut self, value: f64) -> Self {
        self.certificate_lifetime_percent = Some(value);
        self
    }

    pub fn client_region(mut self, value: String) -> Self {
        self.client_region = Some(value);
        self
    }

    pub fn client_service(mut self, value: String) -> Self {
        self.client_service = Some(value);
        self
    }

    pub fn days_until_expiration(mut self, value: i64) -> Self {
        self.days_until_expiration = Some(value);
        self
    }

    pub fn dns_query(mut self, value: String) -> Self {
        self.dns_query = Some(value);
        self
    }

    pub fn dns_server(mut self, value: String) -> Self {
        self.dns_server = Some(value);
        self
    }

    pub fn domain_name(mut self, value: String) -> Self {
        self.domain_name = Some(value);
        self
    }

    pub fn failure_magnitude(mut self, value: i64) -> Self {
        self.failure_magnitude = Some(value);
        self
    }

    pub fn failure_rate(mut self, value: f64) -> Self {
        self.failure_rate = Some(value);
        self
    }

    pub fn failure_type(
        mut self,
        value: crate::datadogV2::model::NetworkHealthInsightFailureType,
    ) -> Self {
        self.failure_type = Some(value);
        self
    }

    pub fn loadbalancer_id(mut self, value: String) -> Self {
        self.loadbalancer_id = Some(value);
        self
    }

    pub fn server_region(mut self, value: String) -> Self {
        self.server_region = Some(value);
        self
    }

    pub fn server_service(mut self, value: String) -> Self {
        self.server_service = Some(value);
        self
    }

    pub fn total_requests(mut self, value: i64) -> Self {
        self.total_requests = Some(value);
        self
    }

    pub fn traffic_volume(
        mut self,
        value: crate::datadogV2::model::NetworkHealthInsightTrafficVolume,
    ) -> Self {
        self.traffic_volume = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::NetworkHealthInsightCategory) -> Self {
        self.type_ = Some(value);
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

impl Default for NetworkHealthInsightAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NetworkHealthInsightAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NetworkHealthInsightAttributesVisitor;
        impl<'a> Visitor<'a> for NetworkHealthInsightAttributesVisitor {
            type Value = NetworkHealthInsightAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut certificate_id: Option<String> = None;
                let mut certificate_lifetime_percent: Option<f64> = None;
                let mut client_region: Option<String> = None;
                let mut client_service: Option<String> = None;
                let mut days_until_expiration: Option<i64> = None;
                let mut dns_query: Option<String> = None;
                let mut dns_server: Option<String> = None;
                let mut domain_name: Option<String> = None;
                let mut failure_magnitude: Option<i64> = None;
                let mut failure_rate: Option<f64> = None;
                let mut failure_type: Option<
                    crate::datadogV2::model::NetworkHealthInsightFailureType,
                > = None;
                let mut loadbalancer_id: Option<String> = None;
                let mut server_region: Option<String> = None;
                let mut server_service: Option<String> = None;
                let mut total_requests: Option<i64> = None;
                let mut traffic_volume: Option<
                    crate::datadogV2::model::NetworkHealthInsightTrafficVolume,
                > = None;
                let mut type_: Option<crate::datadogV2::model::NetworkHealthInsightCategory> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "certificate_id" => {
                            if v.is_null() {
                                continue;
                            }
                            certificate_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "certificate_lifetime_percent" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            certificate_lifetime_percent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_region" => {
                            if v.is_null() {
                                continue;
                            }
                            client_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_service" => {
                            if v.is_null() {
                                continue;
                            }
                            client_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "days_until_expiration" => {
                            if v.is_null() {
                                continue;
                            }
                            days_until_expiration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dns_query" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dns_server" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_server = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain_name" => {
                            if v.is_null() {
                                continue;
                            }
                            domain_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_magnitude" => {
                            if v.is_null() {
                                continue;
                            }
                            failure_magnitude =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            failure_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_type" => {
                            if v.is_null() {
                                continue;
                            }
                            failure_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _failure_type) = failure_type {
                                match _failure_type {
                                    crate::datadogV2::model::NetworkHealthInsightFailureType::UnparsedObject(_failure_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "loadbalancer_id" => {
                            if v.is_null() {
                                continue;
                            }
                            loadbalancer_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "server_region" => {
                            if v.is_null() {
                                continue;
                            }
                            server_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "server_service" => {
                            if v.is_null() {
                                continue;
                            }
                            server_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            total_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traffic_volume" => {
                            if v.is_null() {
                                continue;
                            }
                            traffic_volume =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::NetworkHealthInsightCategory::UnparsedObject(_type_) => {
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

                let content = NetworkHealthInsightAttributes {
                    account_id,
                    certificate_id,
                    certificate_lifetime_percent,
                    client_region,
                    client_service,
                    days_until_expiration,
                    dns_query,
                    dns_server,
                    domain_name,
                    failure_magnitude,
                    failure_rate,
                    failure_type,
                    loadbalancer_id,
                    server_region,
                    server_service,
                    total_requests,
                    traffic_volume,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NetworkHealthInsightAttributesVisitor)
    }
}
