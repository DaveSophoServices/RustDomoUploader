use serde::Deserialize;
use domo::public::Client;
use std::fs;
use futures::executor::block_on;
use rust_active_campaign;

mod domo_util;

#[derive(Debug,Deserialize)]
struct DomoConfigFile {
    client_id: String,
    secret: String,
}
#[derive(Debug,Deserialize)]
struct ActiveCampaignConfigFile {
    namespace: String,
    token: String,
}
#[derive(Debug,Deserialize)]
struct SyncConfigFile {
    campaigns: bool,
}
#[derive(Debug,Deserialize)]
struct ConfigFile {
    domo: DomoConfigFile,
    activecampaign: ActiveCampaignConfigFile,
    sync: SyncConfigFile,
}

fn main() {
    let config = read_config();

    let dc = Client::new("https://api.domo.com", &config.domo.client_id, &config.domo.secret);
    
    let ds_promise = domo_util::find_or_create_campaign_dataset(&dc);

    let ds = block_on(ds_promise).expect("Failed to find dataset for campaigns");
    println!("Dataset: {}", ds);
}

fn read_config() -> ConfigFile {
    let contents = match fs::read_to_string("config.toml") {
	Ok(x) => x,
	Err(e) => panic!("Please create config file 'config.toml' with values for domo: (client_id and secret), activecampaign: (namespace and token), sync: (campaigns). \n\nError: {}", e)
    };
    return match toml::from_str(&contents) {
	Ok(x) => x,
	Err(e) => panic!(
	    r#"Contents of config.toml should be 
[domo]
client_id="..."
secret="..."
[activecampaign]
namespace="..."
token="...".
[sync]
campaigns=true/false
 
Error: {}"#, e),
    };   
}

