use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::{any::Any, env, io};
use tokio::sync::OnceCell;

/// ConfigFields
/// ```
/// DatabaseUrl: String // postgres compatible connection string
/// DatabaseCertPath: String // absolute path to root CA certificate
/// StaticFilesDir: String // absolute path to serve static files from
/// JwtSecret: String // for creating jwt keys
/// RegisterUserSecret: String // verification that user is allowed to create another users
/// DatabaseCertProvided: bool // does connection to database require certificate?
/// UserActionDaily: u8 // how many actions like emails and purchases can user do perform in 24 hours
/// MailerSendToken: String // token for sending emails via mailersend.com
/// ```
#[derive(Clone)]
pub enum ConfigField {
    DatabaseUrl,
    DatabaseCertPath,
    StaticFilesDir,
    StaticFileUrl,
    JwtSecret,
    RegisterUserSecret,
    DatabaseCertProvided,
    UserActionDaily,
    MailerSendToken,
    AppBaseUrl,
}

impl ConfigField {
    pub fn to_str(&self) -> &str {
        match self {
            ConfigField::DatabaseUrl => "database_url",
            ConfigField::DatabaseCertPath => "database_cert_path",
            ConfigField::StaticFilesDir => "static_files_dir",
            ConfigField::StaticFileUrl => "static_files_url",
            ConfigField::JwtSecret => "jwt_secret",
            ConfigField::RegisterUserSecret => "register_user_secret",
            ConfigField::DatabaseCertProvided => "database_cert_provided",
            ConfigField::UserActionDaily => "user_action_daily",
            ConfigField::MailerSendToken => "mailer_send_token",
            ConfigField::AppBaseUrl => "app_base_url",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_cert_path: String,
    pub static_files_dir: String,
    pub static_files_url: String,
    pub jwt_secret: String,
    pub register_user_secret: String,
    pub database_cert_provided: bool,
    pub user_action_daily: u8,
    pub mailer_send_token: String,
    pub app_base_url: String,
}

impl Config {
    pub fn get_field<T: 'static>(&self, field: ConfigField) -> Result<T, io::Error>
    where
        T: Any + Clone,
    {
        let value: Box<dyn Any + Send> = match field {
            ConfigField::DatabaseUrl => Box::new(self.database_url.clone()),
            ConfigField::DatabaseCertPath => Box::new(self.database_cert_path.clone()),
            ConfigField::StaticFilesDir => Box::new(self.static_files_dir.clone()),
            ConfigField::StaticFileUrl => Box::new(self.static_files_url.clone()),
            ConfigField::JwtSecret => Box::new(self.jwt_secret.clone()),
            ConfigField::RegisterUserSecret => Box::new(self.register_user_secret.clone()),
            ConfigField::DatabaseCertProvided => Box::new(self.database_cert_provided),
            ConfigField::UserActionDaily => Box::new(self.user_action_daily),
            ConfigField::MailerSendToken => Box::new(self.mailer_send_token.clone()),
            ConfigField::AppBaseUrl => Box::new(self.app_base_url.clone()),
        };

        if let Some(result) = value.downcast_ref::<T>() {
            Ok(result.clone())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to get config property {}", &field.to_str()),
            ))
        }
    }
}

lazy_static! {
    pub static ref CONFIG: OnceCell<Arc<Config>> = OnceCell::new();
}

pub async fn init() -> Result<(), io::Error> {
    dotenv().ok();
    debug!(target: "cfg", ".env file loaded");
    let missing_required_error = "is required config property";

    let mut field = ConfigField::DatabaseUrl.to_str();
    let database_url = env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::DatabaseCertPath.to_str();
    let database_cert_path =
        env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::StaticFilesDir.to_str();
    let static_files_dir =
        env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::StaticFileUrl.to_str();
    let static_files_url =
        env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::JwtSecret.to_str();
    let jwt_secret = env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::RegisterUserSecret.to_str();
    let register_user_secret =
        env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::DatabaseCertProvided.to_str();
    let database_cert_provided_string = env::var(&field);
    let database_cert_provided = match database_cert_provided_string {
        Ok(value) => {
            if value == "true" {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };

    field = ConfigField::UserActionDaily.to_str();
    let user_action_limit =
        env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));
    let user_action_num = user_action_limit
        .parse::<u8>()
        .expect(&format!("{} is not valid u8", &field));
    let user_action_daily: u8 = user_action_num;
		debug!(target: "app", "config:load -  user_action_num {}", user_action_num);

    field = ConfigField::MailerSendToken.to_str();
    let mailer_send_token =
        env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    field = ConfigField::AppBaseUrl.to_str();
    let app_base_url = env::var(&field).expect(&format!("{} {}", &field, &missing_required_error));

    let config = Arc::new(Config {
        database_url,
        database_cert_path,
        static_files_dir,
        static_files_url,
        jwt_secret,
        register_user_secret,
        database_cert_provided,
        user_action_daily,
        mailer_send_token,
        app_base_url,
    });
    debug!(target: "cfg", "config instance created");

    CONFIG
        .set(config)
        .expect("Failed to set config as static reference");

    Ok(())
}

pub async fn get<T>(field: ConfigField) -> Result<T, io::Error>
where
    T: 'static + Any + Clone,
{
    let config_ref = CONFIG
        .get()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Config does not exist"))?;

    let config = config_ref.as_ref();

    let field_clone = field.clone();
    let result = config.get_field::<T>(field);

    match &result {
        Ok(_) => {
            debug!(
                target: "cfg", "Successfully retrieved config value for field {}",
                field_clone.to_str()
            );
        }
        Err(e) => {
            error!(target: "cfg", "Failed to retrieve config value: {:?}", e);
        }
    };

    result
}

pub fn get_sync<T>(field: ConfigField) -> Result<T, io::Error>
where
    T: 'static + Any + Clone,
{
    let config_ref = CONFIG
        .get()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Config does not exist"))?;

    let config = config_ref.as_ref();

    let field_clone = field.clone();
    let result = config.get_field::<T>(field);

    match &result {
        Ok(_) => {
            debug!(
                target: "cfg", "Successfully retrieved config value for field {}",
                field_clone.to_str()
            );
        }
        Err(e) => {
            error!(target: "cfg", "Failed to retrieve config value: {:?}", e);
        }
    };

    result
}
