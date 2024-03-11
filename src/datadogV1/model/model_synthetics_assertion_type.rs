// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsAssertionType {
    BODY,
    HEADER,
    STATUS_CODE,
    CERTIFICATE,
    RESPONSE_TIME,
    PROPERTY,
    RECORD_EVERY,
    RECORD_SOME,
    TLS_VERSION,
    MIN_TLS_VERSION,
    LATENCY,
    PACKET_LOSS_PERCENTAGE,
    PACKETS_RECEIVED,
    NETWORK_HOP,
    RECEIVED_MESSAGE,
    GRPC_HEALTHCHECK_STATUS,
    GRPC_METADATA,
    GRPC_PROTO,
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

impl Serialize for SyntheticsAssertionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "body" => Self::BODY,
            "header" => Self::HEADER,
            "statusCode" => Self::STATUS_CODE,
            "certificate" => Self::CERTIFICATE,
            "responseTime" => Self::RESPONSE_TIME,
            "property" => Self::PROPERTY,
            "recordEvery" => Self::RECORD_EVERY,
            "recordSome" => Self::RECORD_SOME,
            "tlsVersion" => Self::TLS_VERSION,
            "minTlsVersion" => Self::MIN_TLS_VERSION,
            "latency" => Self::LATENCY,
            "packetLossPercentage" => Self::PACKET_LOSS_PERCENTAGE,
            "packetsReceived" => Self::PACKETS_RECEIVED,
            "networkHop" => Self::NETWORK_HOP,
            "receivedMessage" => Self::RECEIVED_MESSAGE,
            "grpcHealthcheckStatus" => Self::GRPC_HEALTHCHECK_STATUS,
            "grpcMetadata" => Self::GRPC_METADATA,
            "grpcProto" => Self::GRPC_PROTO,
            "connection" => Self::CONNECTION,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
