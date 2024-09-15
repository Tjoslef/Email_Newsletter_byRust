use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use sqlx::postgres::{PgPool, PgPoolOptions};
use zero2prod::telemetry::{get_subcriber, int_subscriber};
#[tokio::main]
async fn main() ->Result<(), std::io::Error>{

   let subscriber = get_subcriber("zero2prod".into(), "info".into(), std::io::stdout);
    int_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
   let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.connect_options());
    let address = format!("{}:{}",configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(&address)?;
    run(listener,connection_pool)?.await?;
    Ok(())
}

