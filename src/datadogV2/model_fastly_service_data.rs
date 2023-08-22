// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyServiceData {
    /* Attributes object for Fastly service requests. */
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: FastlyServiceAttributes,
    /* The ID of the Fastly service. */
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /* The JSON:API type for this API. Should always be `fastly-services`. */
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: FastlyServiceType,
}
