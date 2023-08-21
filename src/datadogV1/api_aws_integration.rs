// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateAWSAccountParams is a struct for passing parameters to the method [`CreateAWSAccount`]
#[derive(Clone, Debug)]
pub struct CreateAWSAccountParams {
    /* AWS Request Object */
    pub body: AWSAccount,
}

// CreateAWSTagFilterParams is a struct for passing parameters to the method [`CreateAWSTagFilter`]
#[derive(Clone, Debug)]
pub struct CreateAWSTagFilterParams {
    /* Set an AWS tag filter using an `aws_account_identifier`, `namespace`, and filtering string.
Namespace options are `application_elb`, `elb`, `lambda`, `network_elb`, `rds`, `sqs`, and `custom`. */
    pub body: AWSTagFilterCreateRequest,
}

// CreateNewAWSExternalIDParams is a struct for passing parameters to the method [`CreateNewAWSExternalID`]
#[derive(Clone, Debug)]
pub struct CreateNewAWSExternalIDParams {
    /* Your Datadog role delegation name.
For more information about your AWS account Role name,
see the [Datadog AWS integration configuration info](https://docs.datadoghq.com/integrations/amazon_web_services/#setup). */
    pub body: AWSAccount,
}

// DeleteAWSAccountParams is a struct for passing parameters to the method [`DeleteAWSAccount`]
#[derive(Clone, Debug)]
pub struct DeleteAWSAccountParams {
    /* AWS request object */
    pub body: AWSAccountDeleteRequest,
}

// DeleteAWSTagFilterParams is a struct for passing parameters to the method [`DeleteAWSTagFilter`]
#[derive(Clone, Debug)]
pub struct DeleteAWSTagFilterParams {
    /* Delete a tag filtering entry for a given AWS account and `dd-aws` namespace. */
    pub body: AWSTagFilterDeleteRequest,
}

// ListAWSAccountsParams is a struct for passing parameters to the method [`ListAWSAccounts`]
#[derive(Clone, Debug)]
pub struct ListAWSAccountsParams {
    /* Only return AWS accounts that matches this `account_id`. */
    pub account_id: String,
    /* Only return AWS accounts that matches this role_name. */
    pub role_name: String,
    /* Only return AWS accounts that matches this `access_key_id`. */
    pub access_key_id: String,
}

// ListAWSTagFiltersParams is a struct for passing parameters to the method [`ListAWSTagFilters`]
#[derive(Clone, Debug)]
pub struct ListAWSTagFiltersParams {
    /* Only return AWS filters that matches this `account_id`. */
    pub account_id: String,
}

// UpdateAWSAccountParams is a struct for passing parameters to the method [`UpdateAWSAccount`]
#[derive(Clone, Debug)]
pub struct UpdateAWSAccountParams {
    /* AWS request object */
    pub body: AWSAccount,
    /* Only return AWS accounts that matches this `account_id`. */
    pub account_id: String,
    /* Only return AWS accounts that match this `role_name`.
Required if `account_id` is specified. */
    pub role_name: String,
    /* Only return AWS accounts that matches this `access_key_id`.
Required if none of the other two options are specified. */
    pub access_key_id: String,
}




/// CreateAWSAccountError is a struct for typed errors of method [`CreateAWSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAWSTagFilterError is a struct for typed errors of method [`CreateAWSTagFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSTagFilterError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateNewAWSExternalIDError is a struct for typed errors of method [`CreateNewAWSExternalID`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewAWSExternalIDError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSAccountError is a struct for typed errors of method [`DeleteAWSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSTagFilterError is a struct for typed errors of method [`DeleteAWSTagFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSTagFilterError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAWSAccountsError is a struct for typed errors of method [`ListAWSAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSAccountsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAWSTagFiltersError is a struct for typed errors of method [`ListAWSTagFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSTagFiltersError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAvailableAWSNamespacesError is a struct for typed errors of method [`ListAvailableAWSNamespaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailableAWSNamespacesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAWSAccountError is a struct for typed errors of method [`UpdateAWSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAWSAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}