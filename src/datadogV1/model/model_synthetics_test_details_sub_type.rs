// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestDetailsSubType {
    #[serde(rename = "http")]
    HTTP,
    #[serde(rename = "ssl")]
    SSL,
    #[serde(rename = "tcp")]
    TCP,
    #[serde(rename = "dns")]
    DNS,
    #[serde(rename = "multi")]
    MULTI,
    #[serde(rename = "icmp")]
    ICMP,
    #[serde(rename = "udp")]
    UDP,
    #[serde(rename = "websocket")]
    WEBSOCKET,
    #[serde(rename = "grpc")]
    GRPC,
}

impl ToString for SyntheticsTestDetailsSubType {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP => String::from("http"),
            Self::SSL => String::from("ssl"),
            Self::TCP => String::from("tcp"),
            Self::DNS => String::from("dns"),
            Self::MULTI => String::from("multi"),
            Self::ICMP => String::from("icmp"),
            Self::UDP => String::from("udp"),
            Self::WEBSOCKET => String::from("websocket"),
            Self::GRPC => String::from("grpc"),
        }
    }
}
