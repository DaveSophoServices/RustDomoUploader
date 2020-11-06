use rust_active_campaign::{ACSession,campaign::Campaigns};
mod error;
use error::Error;

pub async fn get_campaigns(ac:&ACSession) -> Result<Campaigns, Error> {
    ac.list_all_campaigns().map_err(|x| Error::StrError(format!("{}",x)))
}
