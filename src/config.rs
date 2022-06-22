#[warn(clippy::all)]
use clap::Parser;
use colored::*;

#[derive(clap::Parser, Debug, PartialEq, Eq)]
pub struct Config {
    /// Choose the level of logs printed on stdout terminal screen
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Database location. Local or remote
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// Exposed port to connect on the Database
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// User attached to the Database
    #[clap(long)]
    pub db_user: String,
    /// User password if needed to access the Database
    #[clap(long)]
    pub db_user_password: String,
    /// The Database name
    #[clap(long)]
    pub db_name: String,
    /// The exposed port over the WebSocket or localhost
    #[clap(long, default_value = "8080")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        dotenv::dotenv().ok();
        let config = Config::parse();

        if std::env::var("PASETO_SECRET").is_err() {
            println!(
                "{}",
                "HINT: add a .env file or call PASETO_SECRET manually in shell".yellow()
            );
            panic!("Missing or invalid PASETO_SECRET environment variable.")
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|p| p.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(handle_errors::Error::ParseError)?;

        let db_user = std::env::var("POSTGRES_USER").unwrap_or(config.db_user);
        let db_user_password =
            std::env::var("POSTGRES_USER_PASSWORD").unwrap_or(config.db_user_password);
        let db_host = std::env::var("POSTGRES_HOST").unwrap_or(config.db_host);
        let db_port = std::env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        let db_name = std::env::var("POSTGRES_DB").unwrap_or(config.db_name);

        Ok(Config {
            log_level: config.log_level,
            db_host,
            port,
            db_name,
            db_port: db_port
                .parse::<u16>()
                .map_err(handle_errors::Error::ParseError)?,
            db_user,
            db_user_password,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use color_eyre::Result;

    fn set_env_vars() {
        std::env::set_var("RUST_LOG", "warn");
        std::env::set_var("PASETO_SECRET", "true");
        std::env::set_var("POSTGRES_USER", "postgres");
        std::env::set_var("POSTGRES_PASSWORD", "password");
        std::env::set_var("POSTGRES_HOST", "localhost");
        std::env::set_var("POSTGRES_PORT", "5432");
        std::env::set_var("POSTGRES_DB", "postgres");
    }

    #[test]
    fn check_env_vars() -> Result<()> {
        color_eyre::install()?;

        let check_empty_env = std::panic::catch_unwind(Config::new);
        assert!(check_empty_env.is_err());

        set_env_vars();
        let valid_rhs = Config {
            log_level: "warn".to_string(),
            port: 8080,
            db_user: "user".to_string(),
            db_user_password: "password".to_string(),
            db_host: "localhost".to_string(),
            db_name: "postgres".to_string(),
            db_port: 5432,
        };
        let result_config = Config::new().unwrap();

        assert_eq!(result_config, valid_rhs);
        Ok(())
    }
}
