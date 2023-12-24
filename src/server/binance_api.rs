use serde_json::Value;
use std::{env, error::Error};

pub type StatsError = Box<dyn Error + 'static>;
pub type StatsResult<T> = Result<T, StatsError>;

type CandleFetcherror = Box<dyn Error + 'static>;

pub struct Statistics {
    pub current_price: Option<f64>,
}

impl Statistics {
    pub async fn get_stats() -> StatsResult<Statistics> {
        let chandle_data = Self::get_candle().await?;
        let json: Value = serde_json::from_str(&chandle_data)?;

        let mut price_str: String = json.get("price").unwrap().to_string().trim().to_string();

        if price_str.starts_with('"') && price_str.ends_with('"') {
            price_str = price_str[1..price_str.len() - 1].to_string();
        }

        let price_f64: Result<f64, std::num::ParseFloatError> = price_str.parse::<f64>();

        match price_f64 {
            Ok(price) => Ok(Statistics {
                current_price: Some(price),
            }),
            Err(e) => {
                eprintln!("Parse error: {}", e);
                Ok(Statistics {
                    current_price: None,
                })
            }
        }
    }

    async fn get_candle() -> Result<String, CandleFetcherror> {
        let url = env::var("BINANCE_CURRENT_CANDLE_URL")
            .expect("No url to fetch current candle provided");
        let res = reqwest::get(url).await?;

        let response_text = res.text().await?;
        Ok(response_text)
    }

    pub fn has_none(&self) -> bool {
        self.current_price.is_none()
        // || self.another_field.is_none()
    }
}
