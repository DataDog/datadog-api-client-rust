// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team membership attributes
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamAttributes {
    /// The mechanism responsible for provisioning the team relationship.
    /// Possible values: null for added by a user, "service_account" if added by a service account, and "saml_mapping" if provisioned via SAML mapping.
    #[serde(
        rename = "provisioned_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub provisioned_by: Option<Option<String>>,
    /// UUID of the User or Service Account who provisioned this team membership, or null if provisioned via SAML mapping.
    #[serde(
        rename = "provisioned_by_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub provisioned_by_id: Option<Option<String>>,
    /// The user's role within the team
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option")]
    pub role: Option<Option<crate::datadogV2::model::UserTeamRole>>,
}

impl UserTeamAttributes {
    pub fn new() -> UserTeamAttributes {
        UserTeamAttributes {
            provisioned_by: None,
            provisioned_by_id: None,
            role: None,
        }
    }

    pub fn with_provisioned_by(&mut self, value: Option<String>) -> &mut Self {
        self.provisioned_by = Some(value);
        self
    }

    pub fn with_provisioned_by_id(&mut self, value: Option<String>) -> &mut Self {
        self.provisioned_by_id = Some(value);
        self
    }

    pub fn with_role(&mut self, value: Option<crate::datadogV2::model::UserTeamRole>) -> &mut Self {
        self.role = Some(value);
        self
    }
}
impl Default for UserTeamAttributes {
    fn default() -> Self {
        Self::new()
    }
}
