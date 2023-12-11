// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing an organization to create.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCreateBody {
    /// A JSON array of billing type.
    #[deprecated]
    #[serde(rename = "billing")]
    pub billing: Option<Box<crate::datadogV1::model::OrganizationBilling>>,
    /// The name of the new child-organization, limited to 32 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// Subscription definition.
    #[deprecated]
    #[serde(rename = "subscription")]
    pub subscription: Option<Box<crate::datadogV1::model::OrganizationSubscription>>,
}

impl OrganizationCreateBody {
    pub fn new(name: String) -> OrganizationCreateBody {
        #[allow(deprecated)]
        OrganizationCreateBody {
            billing: None,
            name,
            subscription: None,
        }
    }
}
