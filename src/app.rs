use std::{
    collections::{HashMap, HashSet},
    thread::sleep,
    time::Duration,
};

use anyhow::Result;
use truncrate::TruncateToBoundary;

use crate::{mailer::Mailer, scraper::Scraper, subscription_api::SubscriptionApi};

pub struct App {
    pub mailer: Mailer,
    pub subscription_api: SubscriptionApi,
    pub scraper: Scraper,
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for App {
    async fn bind(
        mut self: Box<Self>,
        _addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        self.start().await?;

        Ok(())
    }
}

impl App {
    pub async fn start(&mut self) -> Result<(), shuttle_service::error::Error> {
        loop {
            let mut ad = self.scraper.get_next().await;

            while let Some(sub) = self.subscription_api.get_next().await {
                let mut set = HashSet::new();

                if ad.matches_subscription(&sub) {
                    let email = self
                        .subscription_api
                        .get_user_email(sub.id_user as usize)
                        .await?;

                    set.insert(email);
                }

                for email in set {
                    self.mailer
                        .notify(
                            &email,
                            "A aparut un anunt nou!",
                            &format!("Titlu anunt: {}\n\nLink: {}", ad.contract_object, ad.link,),
                        )
                        .await?;
                }
            }
        }
    }
}
