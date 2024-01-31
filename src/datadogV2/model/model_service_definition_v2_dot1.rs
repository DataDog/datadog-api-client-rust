// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service definition v2.1 for providing service metadata and integrations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1 {
    /// Identifier for a group of related services serving a product feature, which the service is a part of.
    #[serde(rename = "application")]
    pub application: Option<String>,
    /// A list of contacts related to the services.
    #[serde(rename = "contacts")]
    pub contacts: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Contact>>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// A short description of the service.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extensions to v2.1 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Integrations>,
    /// The current life cycle phase of the service.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<String>,
    /// A list of links related to the services.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Link>>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV2Dot1Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team that owns the service. It is used to locate a team defined in Datadog Teams if it exists.
    #[serde(rename = "team")]
    pub team: Option<String>,
    /// Importance of the service.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
}

impl ServiceDefinitionV2Dot1 {
    pub fn new(
        dd_service: String,
        schema_version: crate::datadogV2::model::ServiceDefinitionV2Dot1Version,
    ) -> ServiceDefinitionV2Dot1 {
        ServiceDefinitionV2Dot1 {
            application: None,
            contacts: None,
            dd_service,
            description: None,
            extensions: None,
            integrations: None,
            lifecycle: None,
            links: None,
            schema_version,
            tags: None,
            team: None,
            tier: None,
        }
    }

    pub fn with_application(&mut self, value: String) -> &mut Self {
        self.application = Some(value);
        self
    }

    pub fn with_contacts(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Contact>,
    ) -> &mut Self {
        self.contacts = Some(value);
        self
    }

    pub fn with_description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn with_extensions(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.extensions = Some(value);
        self
    }

    pub fn with_integrations(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1Integrations,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn with_lifecycle(&mut self, value: String) -> &mut Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn with_links(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Link>,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn with_team(&mut self, value: String) -> &mut Self {
        self.team = Some(value);
        self
    }

    pub fn with_tier(&mut self, value: String) -> &mut Self {
        self.tier = Some(value);
        self
    }
}
