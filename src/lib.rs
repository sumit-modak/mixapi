pub mod args;
pub mod routes;
pub mod sys;
pub mod tui;

// temporary test modules
pub mod temp;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    app_name: String,
    default_path: String,
    pub db_conn_str: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            default_path: "sumit-modak/mixapi".into(),
            db_conn_str: String::new(),
        }
    }
}

impl AppConfig {
    pub fn update(&mut self, cfg: &AppConfig) {
        match confy::store(&self.app_name, None, cfg) {
            Ok(_) => println!("app config successfully updated"),
            Err(e) => println!("there was an error updating app config. error: {:?}", e),
        }
    }

    pub fn new() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        AppConfig {
            app_name: app_name.to_string(),
            default_path: "".into(),
            db_conn_str: String::new(),
        }
    }

    pub fn set_config_path(&mut self, config_path: &str) -> &mut Self {
        self.default_path = config_path.into();
        self
    }

    pub fn set_db_conn_str(&mut self, conn_str: &str) -> &mut Self {
        self.db_conn_str = conn_str.to_string();
        self
    }

    pub fn build(&self) -> AppConfig {
        AppConfig {
            app_name: self.app_name.clone(),
            default_path: self.default_path.clone(),
            db_conn_str: self.db_conn_str.clone(),
        }
    }
}
