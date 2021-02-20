use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Event;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{
  api::{Api, ListParams},
  Client,
};
use kube_runtime::{utils::try_flatten_applied, watcher};
use env_var::env_var;
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(),std::io::Error> {
  let ignore_old_entries = env_var!(optional "IGNORE_OLD_ENTRIES",default: "TRUE");
  let output_format = env_var!(optional "OUTPUT_FORMAT", default: "PLAIN");
  let client = Client::try_default().await.expect("getting default client");

  let events: Api<Event> = Api::all(client);
  let lp = ListParams::default();

  let mut ew = try_flatten_applied(watcher(events, lp)).boxed();

  while let Some(event) = ew.try_next().await.unwrap() {
    let creation_timestamp: Time = event.metadata.creation_timestamp.clone().unwrap();
    let creation_seconds = creation_timestamp.0.timestamp();
    let now_seconds = Utc::now().timestamp();
    if ignore_old_entries == "TRUE" && creation_seconds > now_seconds-60 {
      //entry too old
      continue;
    }else{
      if output_format == "PLAIN" {
        println!("[{} {}] {} [{}] {} {} {}"
                  ,creation_timestamp.0.to_rfc3339()
                  ,event.type_.unwrap_or("".to_string())
                  ,event.involved_object.namespace.unwrap_or("".to_string())
                  ,event.involved_object.kind.unwrap_or("".to_string())
                  ,event.involved_object.name.unwrap_or("".to_string())
                  ,event.reason.unwrap_or("".to_string())
                  ,event.message.unwrap_or("".to_string()));
      } else {
        let str = serde_json::to_string(&event.clone()).unwrap();
        println!("{}",str);
      }
    }
  }
  Ok(())
}
