use chrono::prelude::*;
use chrono::Duration;
use env_var::env_var;
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::core::v1::Event;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{
    api::{Api, ListParams},
    Client,
};
use kube_runtime::{watcher, WatchStreamExt};
use std::convert::Infallible;

#[tokio::main]
async fn main() -> Result<(), Infallible> {
    let ignore_old_entries = env_var!(optional "IGNORE_OLD_ENTRIES",default: "TRUE");
    let output_format = env_var!(optional "OUTPUT_FORMAT", default: "PLAIN");
    let client = Client::try_default().await.expect("getting default client");

    let events: Api<Event> = Api::all(client);
    let lp = ListParams::default();

    let ew = watcher(events, lp).applied_objects();

    pin_mut!(ew);

    while let Some(event) = ew.try_next().await.unwrap() {
        let last_minute = Utc::now()
            .checked_sub_signed(Duration::seconds(60))
            .unwrap();
        let last_ts = event.last_timestamp.clone();
        let first_ts = event.first_timestamp.clone();
        let ts: DateTime<Utc> = match last_ts {
            Some(t) => t.0,
            None => first_ts.unwrap_or_else(|| Time(Utc::now())).0,
        };

        if ignore_old_entries == "TRUE" && ts < last_minute {
            //entry too old
            continue;
        } else if output_format == "PLAIN" {
            println!(
                "[{} {}] {} [{}] {} {} {}",
                ts.to_rfc3339(),
                event.type_.unwrap_or_default(),
                event.involved_object.namespace.unwrap_or_default(),
                event.involved_object.kind.unwrap_or_default(),
                event.involved_object.name.unwrap_or_default(),
                event.reason.unwrap_or_default(),
                event.message.unwrap_or_default()
            );
        } else {
            let str = serde_json::to_string(&event.clone()).unwrap();
            println!("{}", str);
        }
    }
    Ok(())
}
