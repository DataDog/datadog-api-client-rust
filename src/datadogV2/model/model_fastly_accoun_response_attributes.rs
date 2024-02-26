// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes object of a Fastly account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccounResponseAttributes {
    /// The name of the Fastly account.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of services belonging to the parent account.
    #[serde(rename = "services")]
    pub services: Option<Vec<crate::datadogV2::model::FastlyService>>,
}

impl FastlyAccounResponseAttributes {
    pub fn new(name: String) -> FastlyAccounResponseAttributes {
        FastlyAccounResponseAttributes {
            name,
            services: None,
        }
    }

    pub fn services(&mut self, value: Vec<crate::datadogV2::model::FastlyService>) -> &mut Self {
        self.services = Some(value);
        self
    }
}
