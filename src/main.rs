use serde::Deserialize;
use domo::public::Client;
use std::fs;
use futures::executor::block_on;
use rust_active_campaign;
use log::{debug,info};

mod domo_util;
mod ac_util;

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
    init_log();

    block_on(do_work(config));
}
async fn do_work(config:ConfigFile) {
    debug!("Creating domo client");
    let dc = Client::new("https://api.domo.com", &config.domo.client_id, &config.domo.secret);
    debug!("Creating Active Campaign client");
    let activecamp = rust_active_campaign::new(&config.activecampaign.namespace, &config.activecampaign.token).unwrap();

    info!("Starting find for campaign dataset");
    let ds_promise = domo_util::find_or_create_campaign_dataset(&dc);
    info!("Starting fetch of Active Campaign campaigns");
    let campaign_promise = ac_util::get_campaigns(&activecamp);

    info!("... waiting futures");
    let (ds,campaigns) = futures::join!(ds_promise,campaign_promise);
    let ds = ds.unwrap();
    debug!("Dataset: {}", ds);

    {
	// need to iterate campaigns into a CSV
	let mut csv = csv::WriterBuilder::new().
	    has_headers(false).
	    from_path("campaigns.csv").unwrap();
	for c in campaigns.unwrap().campaigns.iter() {
	    csv.serialize(&c).unwrap();
	}
    }
    // upload CSV to domo
    domo_util::upload(&dc, &ds, "campaigns.csv").await;
//    debug!("Campaigns: {:#?}", campaigns);
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

fn init_log() {
    use simplelog::*;
    let mut c = ConfigBuilder::new();
    c.add_filter_allow_str("rust_domo_uploader");
    CombinedLogger::init(
	vec![
	    TermLogger::new(
		LevelFilter::Debug,
		c.build(),
		TerminalMode::Mixed),
	    WriteLogger::new(
		LevelFilter::Info,
		c.build(),
		std::fs::File::create("log").unwrap()),
	]).unwrap();
}
