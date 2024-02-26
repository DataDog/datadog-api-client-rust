// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The objects used to delete an AWS tag filter entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSTagFilterDeleteRequest {
    /// The unique identifier of your AWS account.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The namespace associated with the tag filter entry.
    #[serde(rename = "namespace")]
    pub namespace: Option<crate::datadogV1::model::AWSNamespace>,
}

impl AWSTagFilterDeleteRequest {
    pub fn new() -> AWSTagFilterDeleteRequest {
        AWSTagFilterDeleteRequest {
            account_id: None,
            namespace: None,
        }
    }

    pub fn account_id(&mut self, value: String) -> &mut Self {
        self.account_id = Some(value);
        self
    }

    pub fn namespace(&mut self, value: crate::datadogV1::model::AWSNamespace) -> &mut Self {
        self.namespace = Some(value);
        self
    }
}

impl Default for AWSTagFilterDeleteRequest {
    fn default() -> Self {
        Self::new()
    }
}
