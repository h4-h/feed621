#[derive(envconfgen::EnvConfig)]
pub(crate) struct Config {
  database_url: String,
}
