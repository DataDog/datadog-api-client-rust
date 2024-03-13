// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Deprecated - Service definition V1 for providing additional service metadata and integrations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1 {
    /// Contact information about the service.
    #[serde(rename = "contact")]
    pub contact: Option<crate::datadogV2::model::ServiceDefinitionV1Contact>,
    /// Extensions to V1 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A list of external links related to the services.
    #[serde(rename = "external-resources")]
    pub external_resources: Option<Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>>,
    /// Basic information about a service.
    #[serde(rename = "info")]
    pub info: crate::datadogV2::model::ServiceDefinitionV1Info,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::ServiceDefinitionV1Integrations>,
    /// Org related information about the service.
    #[serde(rename = "org")]
    pub org: Option<crate::datadogV2::model::ServiceDefinitionV1Org>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV1Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl ServiceDefinitionV1 {
    pub fn new(
        info: crate::datadogV2::model::ServiceDefinitionV1Info,
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

    pub fn contact(mut self, value: crate::datadogV2::model::ServiceDefinitionV1Contact) -> Self {
        self.contact = Some(value);
        self
    }

    pub fn extensions(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn external_resources(
        mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>,
    ) -> Self {
        self.external_resources = Some(value);
        self
    }

    pub fn integrations(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionV1Integrations,
    ) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn org(mut self, value: crate::datadogV2::model::ServiceDefinitionV1Org) -> Self {
        self.org = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}
