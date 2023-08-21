// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Opsgenie {
    /// Opsgenie instance region.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: ServiceDefinitionV2OpsgenieRegion,
    /// Opsgenie service url.
    #[serde(rename = "service-url", skip_serializing_if = "Option::is_none")]
    pub service_url: String,
}

