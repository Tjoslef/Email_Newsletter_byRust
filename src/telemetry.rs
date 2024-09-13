use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorage, JsonStorageLayer};
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
pub fn get_subcriber(
    name:String,
    env_filter:String


)-> impl Subscriber + Send +Sync{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        std::io::stdout
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
pub fn int_subscriber(subscriber:impl Subscriber +Send + Sync){
 LogTracer::init().expect("Failed to set logger");
 set_global_default(subscriber).expect("Failed to set subscriber")

}
