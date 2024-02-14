// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsAssertionType {
    #[serde(rename = "body")]
    BODY,
    #[serde(rename = "header")]
    HEADER,
    #[serde(rename = "statusCode")]
    STATUS_CODE,
    #[serde(rename = "certificate")]
    CERTIFICATE,
    #[serde(rename = "responseTime")]
    RESPONSE_TIME,
    #[serde(rename = "property")]
    PROPERTY,
    #[serde(rename = "recordEvery")]
    RECORD_EVERY,
    #[serde(rename = "recordSome")]
    RECORD_SOME,
    #[serde(rename = "tlsVersion")]
    TLS_VERSION,
    #[serde(rename = "minTlsVersion")]
    MIN_TLS_VERSION,
    #[serde(rename = "latency")]
    LATENCY,
    #[serde(rename = "packetLossPercentage")]
    PACKET_LOSS_PERCENTAGE,
    #[serde(rename = "packetsReceived")]
    PACKETS_RECEIVED,
    #[serde(rename = "networkHop")]
    NETWORK_HOP,
    #[serde(rename = "receivedMessage")]
    RECEIVED_MESSAGE,
    #[serde(rename = "grpcHealthcheckStatus")]
    GRPC_HEALTHCHECK_STATUS,
    #[serde(rename = "grpcMetadata")]
    GRPC_METADATA,
    #[serde(rename = "grpcProto")]
    GRPC_PROTO,
    #[serde(rename = "connection")]
    CONNECTION,
}

impl ToString for SyntheticsAssertionType {
    fn to_string(&self) -> String {
        match self {
            Self::BODY => String::from("body"),
            Self::HEADER => String::from("header"),
            Self::STATUS_CODE => String::from("statusCode"),
            Self::CERTIFICATE => String::from("certificate"),
            Self::RESPONSE_TIME => String::from("responseTime"),
            Self::PROPERTY => String::from("property"),
            Self::RECORD_EVERY => String::from("recordEvery"),
            Self::RECORD_SOME => String::from("recordSome"),
            Self::TLS_VERSION => String::from("tlsVersion"),
            Self::MIN_TLS_VERSION => String::from("minTlsVersion"),
            Self::LATENCY => String::from("latency"),
            Self::PACKET_LOSS_PERCENTAGE => String::from("packetLossPercentage"),
            Self::PACKETS_RECEIVED => String::from("packetsReceived"),
            Self::NETWORK_HOP => String::from("networkHop"),
            Self::RECEIVED_MESSAGE => String::from("receivedMessage"),
            Self::GRPC_HEALTHCHECK_STATUS => String::from("grpcHealthcheckStatus"),
            Self::GRPC_METADATA => String::from("grpcMetadata"),
            Self::GRPC_PROTO => String::from("grpcProto"),
            Self::CONNECTION => String::from("connection"),
        }
    }
}
