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

    pub fn with_contact(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV1Contact,
    ) -> &mut Self {
        self.contact = Some(value);
        self
    }

    pub fn with_extensions(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.extensions = Some(value);
        self
    }

    pub fn with_external_resources(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>,
    ) -> &mut Self {
        self.external_resources = Some(value);
        self
    }

    pub fn with_integrations(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV1Integrations,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn with_org(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV1Org,
    ) -> &mut Self {
        self.org = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
