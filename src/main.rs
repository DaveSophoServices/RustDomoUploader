use serde::Deserialize;
use std::fs;

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
struct ConfigFile {
    domo: DomoConfigFile,
    activecampaign: ActiveCampaignConfigFile,
}

fn main() {
    let config = read_config();
    println!("{:#?}", config);
    
}

fn read_config() -> ConfigFile {
    let contents = match fs::read_to_string("config.toml") {
	Ok(x) => x,
	Err(e) => panic!("Please create config file 'config.toml' with values for domo: client_id and secret, and activecampaign: namespace and token. \n\nError: {}", e)
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
 
Error: {}"#, e),
    };   
}
