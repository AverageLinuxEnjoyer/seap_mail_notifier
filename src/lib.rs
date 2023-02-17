use app::App;
use shuttle_secrets::SecretStore;

pub(crate) mod advertisment;
pub mod app;
pub(crate) mod mailer;
pub(crate) mod scraper;
pub(crate) mod subscription;
pub(crate) mod subscription_api;

#[shuttle_service::main]
async fn init(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<App, shuttle_service::Error> {
    let msg = "is missing from secret store.";

    Ok(App {
        mailer: mailer::Mailer::new(
            &secret_store
                .get("NAME")
                .unwrap_or_else(|| panic!("NAME {}", msg)),
            &secret_store
                .get("USERNAME")
                .unwrap_or_else(|| panic!("USERNAME {}", msg)),
            &secret_store
                .get("PASSWORD")
                .unwrap_or_else(|| panic!("PASSWORD {}", msg)),
            &secret_store
                .get("RELAY")
                .unwrap_or_else(|| panic!("RELAY {}", msg)),
        )?,
        subscription_api: subscription_api::SubscriptionApi::new(
            &secret_store
                .get("SUBSCRIPTION_API_URL")
                .unwrap_or_else(|| panic!("SUBSCRIPTION_API_URL {}", msg)),
            &secret_store
                .get("SUBSCRIPTION_API_PORT")
                .unwrap_or_else(|| panic!("SUBSCRIPTION_API_PORT {}", msg)),
            50,
        ),
        scraper: scraper::Scraper::new(50, 100, std::time::Duration::from_secs(600)),
    })
}
