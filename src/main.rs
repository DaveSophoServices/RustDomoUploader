use serde::Deserialize;
use domo::public::Client;
use std::fs;
use futures::executor::block_on;
use rust_active_campaign;
use log::{debug,info};
use std::io::Write;

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
    debug!("Dataset: {}", ds.unwrap());

    {
	// need to iterate campaigns into a CSV
	let mut csv = csv::Writer::from_path("campaigns.csv").unwrap();
	for c in campaigns.unwrap().campaigns.iter() {
	    csv.serialize(&c).unwrap();
	}
    }
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
    CombinedLogger::init(
	vec![
	    TermLogger::new(
		LevelFilter::Debug,
		Config::default(),
		TerminalMode::Mixed),
	    WriteLogger::new(
		LevelFilter::Info,
		Config::default(),
		std::fs::File::create("log").unwrap()),
	]).unwrap();
}
