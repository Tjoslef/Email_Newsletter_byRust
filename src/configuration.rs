#[derive(serde::Deserialize)]
pub struct Setting {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize,Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
impl DatabaseSettings {
pub fn connection_string(&self) -> String {
  let eror = format!("postgres://{}:{}@{}:{}/{}",self.username,self.password, self.host, self.port, self.database_name);
     println!("{:}",eror);
        eror
    }
}
pub fn get_configuration() -> Result<Setting,config::ConfigError>{

    let setting = config::Config::builder()
        .add_source(config::File::new("/home/tjoslef/skola/zero2prod/src/configuration.yaml",config::FileFormat::Yaml))
        .build()?;
    setting.try_deserialize::<Setting>()




}
