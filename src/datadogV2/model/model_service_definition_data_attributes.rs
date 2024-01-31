// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service definition attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionDataAttributes {
    /// Metadata about a service definition.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ServiceDefinitionMeta>,
    /// Service definition schema.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::ServiceDefinitionSchema>,
}

impl ServiceDefinitionDataAttributes {
    pub fn new() -> ServiceDefinitionDataAttributes {
        ServiceDefinitionDataAttributes {
            meta: None,
            schema: None,
        }
    }

    pub fn with_meta(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionMeta,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }

    pub fn with_schema(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionSchema,
    ) -> &mut Self {
        self.schema = Some(value);
        self
    }
}
impl Default for ServiceDefinitionDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}
