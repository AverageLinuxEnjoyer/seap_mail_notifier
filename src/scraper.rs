use crate::advertisment::Advertisment;
use anyhow::{Error, Result};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tokio::time::sleep;

pub struct Scraper {
    client: Client,
    ads_per_request: usize,
    request_freq: Duration,
    pub last_ads: Vec<usize>,
    pub new_ads: Vec<usize>,
}

impl Scraper {
    pub fn new(
        ads_per_request: usize,
        remembered_ads_count: usize,
        request_freq: Duration,
    ) -> Self {
        Self {
            client: Client::new(),
            ads_per_request,
            request_freq,
            last_ads: vec![0; remembered_ads_count],
            new_ads: vec![],
        }
    }

    pub async fn get_next(&mut self) -> Advertisment {
        loop {
            while self.new_ads.is_empty() {
                self.new_ads = match self.get_last_ids().await {
                    Ok(ads) if !ads.is_empty() => ads,
                    _ => {
                        sleep(self.request_freq).await;
                        continue;
                    }
                };
            }
            let id = self.new_ads[0];
            self.new_ads.remove(0);
            let ad = self.get_ad(id).await;

            match ad {
                Ok(ad) => return ad,
                Err(err) => {
                    println!("Error when getting ad: {}", err);
                    continue;
                }
            }
        }
    }

    async fn get_last_ids(&mut self) -> Result<Vec<usize>> {
        let resp = self
            .client
            .post("http://e-licitatie.ro/api-pub/AdvNoticeCommon/GetAdvNoticeList/")
            .header("Referer", "http://e-licitatie.ro/pub/adv-notices/list/1")
            .header("Content-Type", "application/json")
            .body(format!(
                "
        {{
            \"pageSize\": {},
            \"publicationDateStart\": \"{}\",
            \"pageIndex\": 0
        }}
        ",
                self.ads_per_request,
                OffsetDateTime::now_utc().format(&Rfc3339)?
            ))
            .send()
            .await?
            .text()
            .await?;

        let v: Value = serde_json::from_str(&resp)?;
        let ads = v
            .get("items")
            .ok_or_else(|| Error::msg("The json doesn't have an 'items' key."))?;

        let ids = ads
            .as_array()
            .ok_or_else(|| Error::msg("The object couldn't be converted into an array."))?
            .iter()
            .map(|val| {
                val.get("advNoticeId")
                    .ok_or_else(|| {
                        Error::msg("The advNoticeId key couldn't be found in the json object.")
                    })
                    .and_then(|id| {
                        id.as_u64().ok_or_else(|| {
                            Error::msg("The advNoticeId value couldn't be converted into a u64.")
                        })
                    })
            })
            .filter_map(|res| res.ok().map(|id| id as usize))
            .collect::<Vec<usize>>();

        let mut ids = ids
            .into_iter()
            .filter(|id| !self.last_ads.contains(id))
            .collect::<Vec<usize>>();

        ids.sort();

        ids.iter().for_each(|id| {
            self.last_ads.remove(0);
            self.last_ads.push(*id);
        });

        Ok(ids)
    }

    pub async fn get_ad(&mut self, id: usize) -> Result<Advertisment> {
        let resp = self
            .client
            .get(format!(
                "http://e-licitatie.ro/api-pub/PUBLICAdvNotice/getForView/{}",
                id
            ))
            .header("Content-Type", "application/json")
            .header(
                "Referer",
                "http://e-licitatie.ro/pub/notices/adv-notices/view/100449529",
            )
            .send()
            .await?
            .text()
            .await?;

        let mut adv: Advertisment = serde_json::from_str(&resp)?;
        adv.link = format!("http://e-licitatie.ro/pub/notices/adv-notices/view/{}", id);

        Ok(adv)
    }
}
