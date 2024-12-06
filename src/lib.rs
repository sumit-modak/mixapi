pub mod args;
pub mod sys;
pub mod tui;
pub mod web;

// temporary test modules
pub mod temp;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AppConfig {
    pub db_conn_str: String,
}

impl AppConfig {
    // pub fn update(&mut self, cfg: &AppConfig) {
    //     match confy::store(&self.app_name, None, cfg) {
    //         Ok(_) => println!("app config successfully updated"),
    //         Err(e) => println!("there was an error updating app config. error: {:?}", e),
    //     }
    // }

    pub fn new() -> Self {
        Self {
            db_conn_str: String::new(),
        }
    }

    pub fn set_db_conn_str(&mut self, conn_str: &str) {
        self.db_conn_str = conn_str.to_string();
    }
}
