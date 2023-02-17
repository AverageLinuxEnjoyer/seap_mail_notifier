use crate::subscription::Subscription;
use anyhow::{Error, Result};
use reqwest::Client;
use serde_json::Value;

pub struct SubscriptionApi {
    client: Client,
    ip_addr: String,
    port: String,
    page_size: usize,

    subscriptions: Vec<Subscription>,
    current_page: usize,
}

impl SubscriptionApi {
    pub fn new(ip_addr: &str, port: &str, page_size: usize) -> Self {
        Self {
            client: Client::new(),
            ip_addr: ip_addr.to_string(),
            port: port.to_string(),
            page_size,

            subscriptions: vec![],
            current_page: 0,
        }
    }

    pub async fn get_next(&mut self) -> Option<Subscription> {
        if self.subscriptions.is_empty() {
            self.subscriptions = match self.get_paginated(self.current_page).await {
                Ok(sub) => sub,
                Err(_) => {
                    return None;
                }
            };

            self.current_page += 1;
        }

        if self.subscriptions.is_empty() {
            self.current_page = 0;
            return None;
        }

        let sub = self.subscriptions[0].clone();
        self.subscriptions.remove(0);

        Some(sub)
    }

    async fn get_paginated(&mut self, page: usize) -> Result<Vec<Subscription>> {
        let url = format!(
            "{}{}/subscriptions?start_index={}&count={}",
            self.ip_addr,
            self.port,
            page * self.page_size,
            self.page_size
        );

        println!("Getting subscriptions from {}", url);

        let resp = self.client.get(url).send().await?.text().await?;

        let resp: Vec<Subscription> = serde_json::from_str(&resp)?;

        Ok(resp)
    }

    pub async fn get_user_email(&mut self, user_id: usize) -> Result<String> {
        let resp = self
            .client
            .get(format!("{}{}/users/{}", self.ip_addr, self.port, user_id))
            .send()
            .await?
            .text()
            .await?;

        let resp: Value = serde_json::from_str(&resp)?;

        let email = resp["email"]
            .as_str()
            .ok_or_else(|| Error::msg("No email field found in json."))?
            .to_string();

        Ok(email)
    }
}
