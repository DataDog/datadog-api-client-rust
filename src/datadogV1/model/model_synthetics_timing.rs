// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing all metrics and their values collected for a Synthetic API test.
/// See the [Synthetic Monitoring Metrics documentation](<https://docs.datadoghq.com/synthetics/metrics/>).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTiming {
    /// The duration in millisecond of the DNS lookup.
    #[serde(rename = "dns")]
    pub dns: Option<f64>,
    /// The time in millisecond to download the response.
    #[serde(rename = "download")]
    pub download: Option<f64>,
    /// The time in millisecond to first byte.
    #[serde(rename = "firstByte")]
    pub first_byte: Option<f64>,
    /// The duration in millisecond of the TLS handshake.
    #[serde(rename = "handshake")]
    pub handshake: Option<f64>,
    /// The time in millisecond spent during redirections.
    #[serde(rename = "redirect")]
    pub redirect: Option<f64>,
    /// The duration in millisecond of the TLS handshake.
    #[serde(rename = "ssl")]
    pub ssl: Option<f64>,
    /// Time in millisecond to establish the TCP connection.
    #[serde(rename = "tcp")]
    pub tcp: Option<f64>,
    /// The overall time in millisecond the request took to be processed.
    #[serde(rename = "total")]
    pub total: Option<f64>,
    /// Time spent in millisecond waiting for a response.
    #[serde(rename = "wait")]
    pub wait: Option<f64>,
}

impl SyntheticsTiming {
    pub fn new() -> SyntheticsTiming {
        SyntheticsTiming {
            dns: None,
            download: None,
            first_byte: None,
            handshake: None,
            redirect: None,
            ssl: None,
            tcp: None,
            total: None,
            wait: None,
        }
    }

    pub fn with_dns(&mut self, value: f64) -> &mut Self {
        self.dns = Some(value);
        self
    }

    pub fn with_download(&mut self, value: f64) -> &mut Self {
        self.download = Some(value);
        self
    }

    pub fn with_first_byte(&mut self, value: f64) -> &mut Self {
        self.first_byte = Some(value);
        self
    }

    pub fn with_handshake(&mut self, value: f64) -> &mut Self {
        self.handshake = Some(value);
        self
    }

    pub fn with_redirect(&mut self, value: f64) -> &mut Self {
        self.redirect = Some(value);
        self
    }

    pub fn with_ssl(&mut self, value: f64) -> &mut Self {
        self.ssl = Some(value);
        self
    }

    pub fn with_tcp(&mut self, value: f64) -> &mut Self {
        self.tcp = Some(value);
        self
    }

    pub fn with_total(&mut self, value: f64) -> &mut Self {
        self.total = Some(value);
        self
    }

    pub fn with_wait(&mut self, value: f64) -> &mut Self {
        self.wait = Some(value);
        self
    }
}
impl Default for SyntheticsTiming {
    fn default() -> Self {
        Self::new()
    }
}
