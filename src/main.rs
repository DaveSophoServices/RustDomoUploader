use serde::Deserialize;
use domo::public::Client;
use std::fs;
use futures::executor::block_on;

#[derive(Debug,Deserialize)]
struct DomoConfigFile {
    host: String,
    client_id: String,
    secret: String,
}
#[derive(Debug,Deserialize)]
struct ActiveCampaignConfigFile {
    namespace: String,
    token: String,
}
#[derive(Debug,Deserialize)]
struct ConfigFile {
    domo: DomoConfigFile,
    activecampaign: ActiveCampaignConfigFile,
}

fn main() {
    let config = read_config();
    println!("{:#?}", config);
    let domo = Client::new(&config.domo.host, &config.domo.client_id, &config.domo.secret);
    block_on(list_datasets(domo));
}

async fn list_datasets(domo: Client) {
    let r = domo.get_datasets(None,None).await.unwrap();
    println!("{:#?}", r);    
}

fn read_config() -> ConfigFile {
    let contents = match fs::read_to_string("config.toml") {
	Ok(x) => x,
	Err(e) => panic!("Please create config file 'config.toml' with values for domo: host, client_id, and secret, and activecampaign: namespace and token. \n\nError: {}", e)
    };
    return match toml::from_str(&contents) {
	Ok(x) => x,
	Err(e) => panic!(
	    r#"Contents of config.toml should be 
[domo]
host="..."
client_id="..."
secret="..."
[activecampaign]
namespace="..."
token="...".
 
Error: {}"#, e),
    };   
}
