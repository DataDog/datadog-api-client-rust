// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    /// A JSON array of billing type.
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none")]
    pub billing: OrganizationBilling,
    /// Date of the organization creation.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// Description of the organization.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// The name of the new child-organization, limited to 32 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The `public_id` of the organization you are operating within.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// A JSON array of settings.
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: OrganizationSettings,
    /// Subscription definition.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: OrganizationSubscription,
    /// Only available for MSP customers. Allows child organizations to be created on a trial plan.
    #[serde(rename = "trial", skip_serializing_if = "Option::is_none")]
    pub trial: bool,
}

