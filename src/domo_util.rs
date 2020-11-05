use domo::public::dataset::{DataSet, Schema, Column};
use domo::public::Client;

mod error;
use error::Error;

pub async fn find_or_create_campaign_dataset(dc: &Client) -> Result<String,Error> {
    let datasetname = "ActiveCampaign-Campaigns";
    let mut myds: Option<String> = None;
    let mut offset:u32 = 0;
    while myds == None {
	// scan the complete list of datasets, calling multiple times scanning
	// the offsets
	let ds = list_datasets(dc,offset).await;
	if ds.len() > 0 {
	    offset += ds.len() as u32; // len will never be more than 50
	    for i in ds.iter() {
		if let Some(n) = &i.name {
		    if n == datasetname {
			myds = Some(i.id.clone().unwrap());
		    }
		}
	    }
	} else {
	    break; // at end of datasets
	}
    }
    return myds.ok_or(Error::NotFound);
}

async fn list_datasets(dc: &Client, offset: u32) -> Vec<DataSet> {
    dc.get_datasets(Some(50),Some(offset)).await.unwrap()
}

async fn create_campaign_dataset(dc: Client, ds_name: &str) -> String {
    macro_rules! col {
	($input:expr, $type:expr) => {
	    Column { name: Some($input.to_string()),
		     column_type: Some($type.to_string())
	    }
	}
    }
    macro_rules! s {
	($input:expr) => {
	    Some($input.to_string())
	}
    }
    let mut ds = DataSet::new();
    ds.name = s!(ds_name);
    ds.schema = Some(Schema {
	columns: Some(vec!(
	    col!("ctype", "STRING"),
	    col!("userid", "STRING"),
	    col!("segmentid", "STRING"),
	    col!("bounceid", "STRING"),
	    col!("realcid", "STRING"),
	    col!("sendid","STRING"),
	    col!("threadid","STRING"),
	    col!("seriesid","STRING"),
	    col!("formid","STRING"),
	    col!("basetemplateid","STRING"),
	    col!("basemessageid","STRING"),
	    col!("addressid","STRING"),
	    col!("source","STRING"),
	    col!("name","STRING"),
	    col!("cdate","DATETIME"),
	    col!("mdate","DATETIME"),
	    col!("sdate","DATETIME"),
	    col!("ldate","DATETIME"),
	    col!("send_amt","LONG"),
	    col!("total_amt","LONG"),
	    col!("opens","LONG"),
	    col!("uniqueopens","LONG"),
	    col!("linkcliks","LONG"),
	    col!("uniquelinkclicks","LONG"),
	    col!("subscriberclicks","LONG"),
	    col!("forwards","LONG"),
	    col!("uniqueforwards","LONG"),
	    col!("hardbounces","LONG"),
	    col!("softbounces","LONG"),
	    col!("unsubscribes","LONG"),
	    col!("unsubreasons","LONG"),
	    col!("updates","LONG"),
	    col!("socialshares","LONG"),
	    col!("replies","LONG"),
	    col!("uniquereplices","LONG"),
	    col!("status", "STRING"),
	    col!("public", "STRING"),
	    col!("mail_transfer", "STRING"),
	    col!("mail_send", "STRING"),
	    col!("mail_cleanup", "STRING"),
	    col!("mailer_log_file", "STRING"),
	    col!("tracklinks", "STRING"),
	    col!("tracklinkanalytics", "STRING"),
	    col!("trackreads", "STRING"),
	    col!("trackreadsanalytics", "STRING"),
	    col!("analytics_campaign_name", "STRING"),
	    col!("tweet", "STRING"),
	    col!("facebook", "STRING"),
	    col!("survey", "STRING"),
	    col!("embed_images", "STRING"),
	    col!("htmlunsub", "STRING"),
	    col!("textunsub", "STRING"),
	    col!("htmlunsubdata", "STRING"),
	    col!("textunsubdata", "STRING"),
	    col!("recurring", "STRING"),
	    col!("willrecur", "STRING"),
	    col!("split_type", "STRING"),
	    col!("split_content", "STRING"),
	    col!("split_offset", "STRING"),
	    col!("split_offset_type", "STRING"),
	    col!("split_winner_messageid", "STRING"),
	    col!("split_winner_awaiting", "STRING"),
	    col!("responder_offset", "STRING"),
	    col!("responder_type", "STRING"),
	    col!("responder_existing", "STRING"),
	    col!("reminder_field", "STRING"),
	    col!("reminder_format", "STRING"),
	    col!("reminder_type", "STRING"),
	    col!("reminder_offset", "STRING"),
	    col!("reminder_offset_type", "STRING"),
	    col!("reminder_offset_sign", "STRING"),
	    col!("reminder_last_cron_run", "STRING"),
	    col!("activerss_interval", "STRING"),
	    col!("activerss_url", "STRING"),
	    col!("activerss_itmes", "STRING"),
	    col!("ip4", "STRING"),
	    col!("laststep", "STRING"),
	    col!("managetext", "STRING"),
	    col!("schedule", "STRING"),
	    col!("scheduleddate", "STRING"),
	    col!("waitpreview", "STRING"),
	    col!("deletestamp", "STRING"),
	    col!("replysys", "STRING"),
	    col!("id", "STRING"),
	    col!("user", "STRING"),
	    col!("automation", "STRING"),
	)),
    });
    let r = dc.post_dataset(ds).await.unwrap();
    return "sdf".to_string();
//     let file = "test.csv";
//     let data = r#""data",1
// "row2",2
// "row3",3
// "#;
//     let data = data.replace("\n", "\r\n");
//     fs::write(file, data).unwrap();
//     let dataset_id = r.id.unwrap();
//     println!("uploading to dataset: {}", dataset_id);
//     // ignore the results of the following. It should return a 204 status
//     // with an empty body, but the domo code tries to parse the return
//     let r = domo.put_dataset_data(&dataset_id,file)
// 	.await;
//     println!("{:#?}", r);
}
