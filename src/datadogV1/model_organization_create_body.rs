// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCreateBody {
    /// A JSON array of billing type.
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none")]
    pub billing: OrganizationBilling,
    /// The name of the new child-organization, limited to 32 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Subscription definition.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: OrganizationSubscription,
}

