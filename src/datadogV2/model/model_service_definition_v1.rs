// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Deprecated - Service definition V1 for providing additional service metadata and integrations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1 {
    /// Contact information about the service.
    #[serde(rename = "contact")]
    pub contact: Option<Box<crate::datadogV2::model::ServiceDefinitionV1Contact>>,
    /// Extensions to V1 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// A list of external links related to the services.
    #[serde(rename = "external-resources")]
    pub external_resources: Option<Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>>,
    /// Basic information about a service.
    #[serde(rename = "info")]
    pub info: Box<crate::datadogV2::model::ServiceDefinitionV1Info>,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<Box<crate::datadogV2::model::ServiceDefinitionV1Integrations>>,
    /// Org related information about the service.
    #[serde(rename = "org")]
    pub org: Option<Box<crate::datadogV2::model::ServiceDefinitionV1Org>>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV1Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl ServiceDefinitionV1 {
    pub fn new(
        info: Box<crate::datadogV2::model::ServiceDefinitionV1Info>,
        schema_version: crate::datadogV2::model::ServiceDefinitionV1Version,
    ) -> ServiceDefinitionV1 {
        ServiceDefinitionV1 {
            contact: None,
            extensions: None,
            external_resources: None,
            info,
            integrations: None,
            org: None,
            schema_version,
            tags: None,
        }
    }
}
