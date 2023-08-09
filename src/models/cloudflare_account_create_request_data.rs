/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CloudflareAccountCreateRequestData : Data object for creating a Cloudflare account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudflareAccountCreateRequestData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::CloudflareAccountCreateRequestAttributes>,
    #[serde(rename = "type")]
    pub r#type: crate::models::CloudflareAccountType,
}

impl CloudflareAccountCreateRequestData {
    /// Data object for creating a Cloudflare account.
    pub fn new(attributes: crate::models::CloudflareAccountCreateRequestAttributes, r#type: crate::models::CloudflareAccountType) -> CloudflareAccountCreateRequestData {
        CloudflareAccountCreateRequestData {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}


