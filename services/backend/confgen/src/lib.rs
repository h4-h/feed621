/// Macros that will generate a config structure for you!
///
/// This macro generates:
/// 1. A structure [`Config`] with private [`String`] fields derived from the `field` variadic value, along with public getter methods.
/// 2. A public [`new`] method that retrieves values from the environment using the names specified in the `var` variadic value.
///
/// WARNING: If a value cannot be fetched from the environment, [`Config::var()`] will panic.
///
/// Syntax: 
/// ```
/// generate_config!{
///   $(field => var),*
/// }
/// ```
///
///
/// Example: 
/// ```
/// generate_config!{
///   database_url => "DATABASE_URL"
/// }
///
/// /* ... */
///
/// let config = Config::new();
/// let db_url = config.database_url();
///
/// /* ... */
/// ```
///
/// Generates:
/// ```
/// pub(crate) struct Config {
///   database_url: String,
/// }
/// 
/// impl Config {
///   pub fn new() -> Self {
///     Self {
///       database_url: Self::var("DATABASE_URL"),
///    }
///  }
///
///  pub fn database_url($self) -> &str { self.database_url.as_str() }  
///
///  fn var(variable: &str) -> String {
///    std::env::var(variable)
///      .unwrap_or_else(|_| panic!("{variable} not found in environment."))
///  }
/// ```
#[macro_export]
macro_rules! generate_config {
    { $( $field:ident => $var:expr ),* } => {
        pub(crate) struct Config {
            $( $field: String ),*
        }

        impl Config {
            pub fn new() -> Self {
                Self {
                    $( $field: Self::var($var) ),*
                }
            }

            $( pub fn $field(&self) -> &str { self.$field.as_str() } )*

            fn var(variable: &str) -> String {
                std::env::var(variable)
                    .unwrap_or_else(|_| panic!("\x1B[0;31m[!] {variable} not found in the environment.\x1B[0m"))
            } 
        }
    };
}
