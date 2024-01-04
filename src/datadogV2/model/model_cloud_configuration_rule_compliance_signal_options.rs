// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// How to generate compliance signals. Useful for cloud_configuration rules only.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleComplianceSignalOptions {
    /// The default activation status.
    #[serde(
        rename = "defaultActivationStatus",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_activation_status: Option<Option<bool>>,
    /// The default group by fields.
    #[serde(
        rename = "defaultGroupByFields",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_group_by_fields: Option<Option<Vec<String>>>,
    /// Whether signals will be sent.
    #[serde(
        rename = "userActivationStatus",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_activation_status: Option<Option<bool>>,
    /// Fields to use to group findings by when sending signals.
    #[serde(
        rename = "userGroupByFields",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_group_by_fields: Option<Option<Vec<String>>>,
}

impl CloudConfigurationRuleComplianceSignalOptions {
    pub fn new() -> CloudConfigurationRuleComplianceSignalOptions {
        CloudConfigurationRuleComplianceSignalOptions {
            default_activation_status: None,
            default_group_by_fields: None,
            user_activation_status: None,
            user_group_by_fields: None,
        }
    }
}