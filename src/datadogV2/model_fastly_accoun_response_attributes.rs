// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccounResponseAttributes {
    /* The name of the Fastly account. */
    #[serde(rename = "name")]
    pub name: String,
    /* A list of services belonging to the parent account. */
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::datadogV2::FastlyService>>,
}

impl FastlyAccounResponseAttributes {
    /* Attributes object of a Fastly account. */
    pub fn new(name: String) -> FastlyAccounResponseAttributes {
        FastlyAccounResponseAttributes {
            name: name,
            services: None,
        }
    }
}
