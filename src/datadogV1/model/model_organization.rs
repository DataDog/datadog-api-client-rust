// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create, edit, and manage organizations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    /// A JSON array of billing type.
    #[deprecated]
    #[serde(rename = "billing")]
    pub billing: Option<crate::datadogV1::model::OrganizationBilling>,
    /// Date of the organization creation.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Description of the organization.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The name of the child organization, limited to 32 characters.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The `public_id` of the organization you are operating within.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// A JSON array of settings.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV1::model::OrganizationSettings>,
    /// Subscription definition.
    #[deprecated]
    #[serde(rename = "subscription")]
    pub subscription: Option<crate::datadogV1::model::OrganizationSubscription>,
    /// Only available for MSP customers. Allows child organizations to be created on a trial plan.
    #[serde(rename = "trial")]
    pub trial: Option<bool>,
}

impl Organization {
    pub fn new() -> Organization {
        #[allow(deprecated)]
        Organization {
            billing: None,
            created: None,
            description: None,
            name: None,
            public_id: None,
            settings: None,
            subscription: None,
            trial: None,
        }
    }

    #[allow(deprecated)]
    pub fn billing(&mut self, value: crate::datadogV1::model::OrganizationBilling) -> &mut Self {
        self.billing = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn settings(&mut self, value: crate::datadogV1::model::OrganizationSettings) -> &mut Self {
        self.settings = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn subscription(
        &mut self,
        value: crate::datadogV1::model::OrganizationSubscription,
    ) -> &mut Self {
        self.subscription = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn trial(&mut self, value: bool) -> &mut Self {
        self.trial = Some(value);
        self
    }
}

impl Default for Organization {
    fn default() -> Self {
        Self::new()
    }
}
