// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Template variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTemplateVariable {
    /// The list of values that the template variable drop-down is limited to.
    #[serde(
        rename = "available_values",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub available_values: Option<Option<Vec<String>>>,
    /// (deprecated) The default value for the template variable on dashboard load. Cannot be used in conjunction with `defaults`.
    #[deprecated]
    #[serde(
        rename = "default",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default: Option<Option<String>>,
    /// One or many default values for template variables on load. If more than one default is specified, they will be unioned together with `OR`. Cannot be used in conjunction with `default`.
    #[serde(rename = "defaults")]
    pub defaults: Option<Vec<String>>,
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// The tag prefix associated with the variable. Only tags with this prefix appear in the variable drop-down.
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option")]
    pub prefix: Option<Option<String>>,
}

impl DashboardTemplateVariable {
    pub fn new(name: String) -> DashboardTemplateVariable {
        #[allow(deprecated)]
        DashboardTemplateVariable {
            available_values: None,
            default: None,
            defaults: None,
            name,
            prefix: None,
        }
    }

    #[allow(deprecated)]
    pub fn available_values(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.available_values = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn default(&mut self, value: Option<String>) -> &mut Self {
        self.default = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn defaults(&mut self, value: Vec<String>) -> &mut Self {
        self.defaults = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn prefix(&mut self, value: Option<String>) -> &mut Self {
        self.prefix = Some(value);
        self
    }
}
