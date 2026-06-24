// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// TLS configuration for the WebSocket source. Use `enabled` for standard `<wss://`> connections, or `with_client_cert` to present a client certificate for mutual TLS.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineWebsocketSourceTls {
    ObservabilityPipelineWebsocketSourceTlsEnabled(
        Box<crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsEnabled>,
    ),
    ObservabilityPipelineWebsocketSourceTlsWithClientCert(
        Box<crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsWithClientCert>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineWebsocketSourceTls {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsEnabled>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineWebsocketSourceTls::ObservabilityPipelineWebsocketSourceTlsEnabled(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsWithClientCert>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineWebsocketSourceTls::ObservabilityPipelineWebsocketSourceTlsWithClientCert(_v));
            }
        }

        return Ok(ObservabilityPipelineWebsocketSourceTls::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
