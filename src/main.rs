use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use tracing::subscriber::set_global_default;
use sqlx::postgres::PgPool;
use zero2prod::telemetry::{get_subcriber, int_subscriber};

#[tokio::main]
async fn main() ->Result<(), std::io::Error>{
   let subscriber = get_subcriber("zero2prod".into(), "info".into(), std::io::stdout);
    int_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
   let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    run(listener,connection_pool)?.await?;
    Ok(())
}

