use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Event;
use kube::{
  api::{Api, ListParams},
  Client,
};
use kube_runtime::{utils::try_flatten_applied, watcher};

#[tokio::main]
async fn main() -> Result<(),std::io::Error> {
  let client = Client::try_default().await.expect("getting default client");

  let events: Api<Event> = Api::all(client);
  let lp = ListParams::default();

  let mut ew = try_flatten_applied(watcher(events, lp)).boxed();

  while let Some(event) = ew.try_next().await.unwrap() {
    println!("[{:?} {}] {}/{}:{} {} {}"
                  ,event.metadata.creation_timestamp.unwrap()
                  ,event.type_.unwrap()
                  ,event.involved_object.namespace.unwrap()
                  ,event.involved_object.kind.unwrap()
                  ,event.involved_object.name.unwrap()
                  ,event.reason.unwrap()
                  ,event.message.unwrap());
  }
  Ok(())
}
