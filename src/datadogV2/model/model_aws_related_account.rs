// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AWS related account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSRelatedAccount {
    /// Attributes for an AWS related account.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::AWSRelatedAccountAttributes>,
    /// The AWS account ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Type of AWS related account.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AWSRelatedAccountType,
}

impl AWSRelatedAccount {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::AWSRelatedAccountType,
    ) -> AWSRelatedAccount {
        AWSRelatedAccount {
            attributes: None,
            id,
            type_,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::AWSRelatedAccountAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}
