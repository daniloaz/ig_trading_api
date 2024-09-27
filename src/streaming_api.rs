use crate::common::{ApiConfig, ExecutionEnvironment};
use crate::rest_api::RestApi;
use lightstreamer_client::ls_client::{LightstreamerClient, Transport};
use lightstreamer_client::subscription::Subscription;
use signal_hook::low_level::signal_name;
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Notify;

const MAX_CONNECTION_ATTEMPTS: u64 = 1;

pub struct StreamingApi {
    ls_client: LightstreamerClient,
    max_connection_attempts: u64,
}

impl StreamingApi {
    pub async fn connect(&mut self) {
        // Create a new Notify instance to send a shutdown signal to the signal handler thread.
        let shutdown_signal = Arc::new(tokio::sync::Notify::new());
        // Spawn a new thread to handle SIGINT and SIGTERM process signals.
        StreamingApi::setup_signal_hook(Arc::clone(&shutdown_signal)).await;
        //
        // Infinite loop that will indefinitely retry failed connections unless
        // a SIGTERM or SIGINT signal is received.
        //
        let mut retry_interval_milis: u64 = 0;
        let mut retry_counter: u64 = 0;
        while retry_counter < self.max_connection_attempts {
            match self.ls_client.connect(Arc::clone(&shutdown_signal)).await {
                Ok(_) => {
                    self.ls_client.disconnect().await;
                    break;
                }
                Err(e) => {
                    println!("Failed to connect: {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_millis(retry_interval_milis)).await;
                    retry_interval_milis = (retry_interval_milis + (200 * retry_counter)).min(5000);
                    retry_counter += 1;
                    println!(
                        "Retrying connection in {} seconds...",
                        format!("{:.2}", retry_interval_milis as f64 / 1000.0)
                    );
                }
            }
        }

        if retry_counter == self.max_connection_attempts {
            println!(
                "Failed to connect after {} retries. Exiting...",
                retry_counter
            );
        } else {
            println!("Exiting orderly from Lightstreamer client...");
        }
    }

    pub async fn new(subscriptions: Vec<Subscription>, config: Option<ApiConfig>) -> Result<Self, Box<dyn Error>> {
        //
        // Load the configuration from config.yaml file if config is not supplied and create a new mutable REST API instance,
        //
        let api_config = config.unwrap_or_else(|| ApiConfig::default());
        let auto_login = api_config.auto_login.unwrap_or(false);
        let max_connection_attempts = api_config.streaming_api_max_connection_attempts.unwrap_or(MAX_CONNECTION_ATTEMPTS);
        //
        // Connect to REST API and authenticate.
        //
        let mut rest_api = match RestApi::new(api_config).await {
            Ok(api) => api,
            Err(e) => {
                return Err(Box::<dyn Error>::from(format!(
                    "Failed to create and initialize REST API: {}",
                    e
                )));
            }
        };
        if !auto_login {
            let _ = rest_api.client.login();
        }

        // Get the CST and X-SECURITY-TOKEN values from the REST API session.
        let (cst, x_security_token) = match StreamingApi::get_tokens(&rest_api) {
            Ok(tokens) => tokens,
            Err(e) => {
                return Err(Box::<dyn Error>::from(format!(
                    "Failed to get CST and X-SECURITY-TOKEN from REST API: {}",
                    e
                )));
            }
        };

        //
        // Create a new Lightstreamer client instance and wrap it in an Arc<Mutex<>> so it can be shared across threads.
        //
        let mut ls_client = LightstreamerClient::new(
            Some(&format!(
                "{}/lightstreamer",
                &rest_api.client.lightstreamer_endpoint
            )),
            None,
            match rest_api.config.execution_environment {
                ExecutionEnvironment::Demo => Some(&rest_api.config.account_number_demo),
                ExecutionEnvironment::Live => Some(&rest_api.config.account_number_live),
            },
            Some(&format!("CST-{}|XST-{}", cst.to_string(), x_security_token)),
        )?;

        for subscription in subscriptions {
            ls_client.subscribe(subscription);
        }

        ls_client
            .connection_options
            .set_forced_transport(Some(Transport::WsStreaming));

        Ok(Self {
            ls_client,
            max_connection_attempts,
        })
    }

    /// Gets the CST and X-SECURITY-TOKEN values from the REST API session.
    fn get_tokens(rest_api: &RestApi) -> Result<(String, String), Box<dyn Error>> {
        //
        // Get auth headers from the REST API session.
        //
        let auth_headers = match rest_api.client.auth_headers {
            Some(ref headers) => headers,
            None => {
                return Err(Box::<dyn Error>::from(
                    "Client not authenticated, auth headers not found.",
                ));
            }
        };
        let cst = match auth_headers.get("cst") {
            Some(cst) => match cst.to_str() {
                Ok(cst) => cst.to_string(),
                Err(_) => {
                    return Err(Box::<dyn Error>::from(
                        "Client not authenticated, CST auth header not found.",
                    ));
                }
            },
            None => {
                return Err(Box::<dyn Error>::from(
                    "Client not authenticated, CST auth header not found.",
                ));
            }
        };
        let x_security_token = match auth_headers.get("x-security-token") {
            Some(x_security_token) => match x_security_token.to_str() {
                Ok(x_security_token) => x_security_token.to_string(),
                Err(_) => {
                    return Err(Box::<dyn Error>::from(
                        "Client not authenticated, X-SECURITY-TOKEN auth header not found.",
                    ));
                }
            },
            None => {
                return Err(Box::<dyn Error>::from(
                    "Client not authenticated, X-SECURITY-TOKEN auth header not found.",
                ));
            }
        };

        // Return the CST and X-SECURITY-TOKEN values.
        Ok((cst, x_security_token))
    }

    /// Sets up a signal hook for SIGINT and SIGTERM.
    ///
    /// Creates a signal hook for the specified signals and spawns a thread to handle them.
    /// When a signal is received, it logs the signal name and performs cleanup before exiting with 0 code
    /// to indicate orderly shutdown.
    ///
    /// # Arguments
    ///
    /// * `full_path` - The full path to the application configuration file.
    ///
    /// # Panics
    ///
    /// The function panics if it fails to create the signal iterator.
    ///
    async fn setup_signal_hook(shutdown_signal: Arc<Notify>) {
        // Create a signal set of signals to be handled and a signal iterator to monitor them.
        let signals = &[SIGINT, SIGTERM];
        let mut signals_iterator = Signals::new(signals).expect("Failed to create signal iterator");

        // Create a new thread to handle signals sent to the process
        tokio::spawn(async move {
            for signal in signals_iterator.forever() {
                println!("Received signal: {}", signal_name(signal).unwrap());
                let _ = shutdown_signal.notify_one();
                break;
            }
        });
    }
}