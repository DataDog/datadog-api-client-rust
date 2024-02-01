// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service definition V2 for providing service metadata and integrations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2 {
    /// A list of contacts related to the services.
    #[serde(rename = "contacts")]
    pub contacts: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Contact>>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// Experimental feature. A Team handle that matches a Team in the Datadog Teams product.
    #[serde(rename = "dd-team")]
    pub dd_team: Option<String>,
    /// A list of documentation related to the services.
    #[serde(rename = "docs")]
    pub docs: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Doc>>,
    /// Extensions to V2 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::ServiceDefinitionV2Integrations>,
    /// A list of links related to the services.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Link>>,
    /// A list of code repositories related to the services.
    #[serde(rename = "repos")]
    pub repos: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Repo>>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV2Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team that owns the service.
    #[serde(rename = "team")]
    pub team: Option<String>,
}

impl ServiceDefinitionV2 {
    pub fn new(
        dd_service: String,
        schema_version: crate::datadogV2::model::ServiceDefinitionV2Version,
    ) -> ServiceDefinitionV2 {
        ServiceDefinitionV2 {
            contacts: None,
            dd_service,
            dd_team: None,
            docs: None,
            extensions: None,
            integrations: None,
            links: None,
            repos: None,
            schema_version,
            tags: None,
            team: None,
        }
    }

    pub fn contacts(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Contact>,
    ) -> &mut Self {
        self.contacts = Some(value);
        self
    }

    pub fn dd_team(&mut self, value: String) -> &mut Self {
        self.dd_team = Some(value);
        self
    }

    pub fn docs(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Doc>,
    ) -> &mut Self {
        self.docs = Some(value);
        self
    }

    pub fn extensions(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.extensions = Some(value);
        self
    }

    pub fn integrations(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Integrations,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn links(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Link>,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn repos(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Repo>,
    ) -> &mut Self {
        self.repos = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn team(&mut self, value: String) -> &mut Self {
        self.team = Some(value);
        self
    }
}
