// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DnsMetricKey {
    DNS_TOTAL_REQUESTS,
    DNS_FAILURES,
    DNS_SUCCESSFUL_RESPONSES,
    DNS_FAILED_RESPONSES,
    DNS_TIMEOUTS,
    DNS_RESPONSES_NXDOMAIN,
    DNS_RESPONSES_SERVFAIL,
    DNS_RESPONSES_OTHER,
    DNS_SUCCESS_LATENCY_PERCENTILE,
    DNS_FAILURE_LATENCY_PERCENTILE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DnsMetricKey {
    fn to_string(&self) -> String {
        match self {
            Self::DNS_TOTAL_REQUESTS => String::from("dns_total_requests"),
            Self::DNS_FAILURES => String::from("dns_failures"),
            Self::DNS_SUCCESSFUL_RESPONSES => String::from("dns_successful_responses"),
            Self::DNS_FAILED_RESPONSES => String::from("dns_failed_responses"),
            Self::DNS_TIMEOUTS => String::from("dns_timeouts"),
            Self::DNS_RESPONSES_NXDOMAIN => String::from("dns_responses.nxdomain"),
            Self::DNS_RESPONSES_SERVFAIL => String::from("dns_responses.servfail"),
            Self::DNS_RESPONSES_OTHER => String::from("dns_responses.other"),
            Self::DNS_SUCCESS_LATENCY_PERCENTILE => String::from("dns_success_latency_percentile"),
            Self::DNS_FAILURE_LATENCY_PERCENTILE => String::from("dns_failure_latency_percentile"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DnsMetricKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for DnsMetricKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "dns_total_requests" => Self::DNS_TOTAL_REQUESTS,
            "dns_failures" => Self::DNS_FAILURES,
            "dns_successful_responses" => Self::DNS_SUCCESSFUL_RESPONSES,
            "dns_failed_responses" => Self::DNS_FAILED_RESPONSES,
            "dns_timeouts" => Self::DNS_TIMEOUTS,
            "dns_responses.nxdomain" => Self::DNS_RESPONSES_NXDOMAIN,
            "dns_responses.servfail" => Self::DNS_RESPONSES_SERVFAIL,
            "dns_responses.other" => Self::DNS_RESPONSES_OTHER,
            "dns_success_latency_percentile" => Self::DNS_SUCCESS_LATENCY_PERCENTILE,
            "dns_failure_latency_percentile" => Self::DNS_FAILURE_LATENCY_PERCENTILE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
