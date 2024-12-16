// Get all CSM Serverless Agents returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_agents::CSMAgentsAPI;
use datadog_api_client::datadogV2::api_csm_agents::ListAllCSMServerlessAgentsOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CSMAgentsAPI::with_config(configuration);
    let resp = api
        .list_all_csm_serverless_agents(ListAllCSMServerlessAgentsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
