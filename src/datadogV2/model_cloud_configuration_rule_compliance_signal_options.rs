// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleComplianceSignalOptions {
    /// The default activation status.
    #[serde(rename = "defaultActivationStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub default_activation_status: Option<Bool>,
    /// The default group by fields.
    #[serde(rename = "defaultGroupByFields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub default_group_by_fields: datadog.NullableList[String],
    /// Whether signals will be sent.
    #[serde(rename = "userActivationStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub user_activation_status: Option<Bool>,
    /// Fields to use to group findings by when sending signals.
    #[serde(rename = "userGroupByFields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub user_group_by_fields: datadog.NullableList[String],
}

