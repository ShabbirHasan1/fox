use reqwest::Client;
use crate::dydx::{InternalAccount, Position, Markets, OrderType, TradeData, self};
use chrono::{Utc, Duration};

use std::collections::HashMap;

impl InternalAccount {
 
    pub async fn accounts(&self, testnet: bool) -> anyhow::Result<String> {
       
        let url = {
            if testnet { url::Url::parse("https://api.stage.dydx.exchange/v3/accounts").unwrap() }
            else { url::Url::parse("https://api.dydx.exchange/v3/accounts").unwrap() }
        };

        let (signature, timestamp) = self.signature_for("/v3/accounts".to_string(), "GET".to_string(), None);
        
        let client = Client::new();
        let response = client.get(url)
            .header("dydx-signature".to_string(), signature)
            .header("dydx-api-key".to_string(), self.api_key.clone())
            .header("dydx-timestamp".to_string(), timestamp)
            .header("dydx-passphrase".to_string(), self.api_passphrase.clone())
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    pub async fn positions(&self, testnet: bool) -> anyhow::Result<String> {
       
        let url = {
            if testnet { url::Url::parse("https://api.stage.dydx.exchange/v3/positions").unwrap() }
            else { url::Url::parse("https://api.dydx.exchange/v3/positions").unwrap() }
        };

        let (signature, timestamp) = self.signature_for("/v3/positions".to_string(), "GET".to_string(), None);
        
        let client = Client::new();
        let response = client.get(url)
            .header("dydx-signature".to_string(), signature)
            .header("dydx-api-key".to_string(), self.api_key.clone())
            .header("dydx-timestamp".to_string(), timestamp)
            .header("dydx-passphrase".to_string(), self.api_passphrase.clone())
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn open_order(
        &self, 
        market: Markets, 
        position: Position, 
        order_type: OrderType, 
        fee: String, 
        client_id: String, 
        position_id: u64, 
        post_only: bool, 
        trade_data: TradeData,
        testnet: bool
    ) -> anyhow::Result<String> {
        
        let url = {
            if testnet { url::Url::parse("https://api.stage.dydx.exchange/v3/orders").unwrap() }
            else { url::Url::parse("https://api.dydx.exchange/v3/orders").unwrap() }
        };

        let time_now = Utc::now();
        let expiration_time = time_now + Duration::seconds(63);
        let expiration_seconds = expiration_time.timestamp() as u64;
        
        let order_hash = dydx::crypto::order_hash(client_id.clone(), &position, market, fee.clone(), expiration_seconds, position_id, testnet);
        let order_signature = dydx::crypto::sign(order_hash, self.stark_private_key());
        
        let mut data = HashMap::new();
        data.insert("market".to_string(), format!("{}", market));
        data.insert("side".to_string(), format!("{:?}", position.side));
        data.insert("type".to_string(), format!("{:?}", order_type));
        data.insert("postOnly".to_string(), format!("{}", post_only));
        data.insert("size".to_string(), position.size);
        data.insert("price".to_string(), position.open_price.clone());
        data.insert("limitFee".to_string(), fee);
        data.insert("expiration".to_string(), format!("{:?}", expiration_time));
        data.insert("clientId".to_string(), client_id);
        if let Some(tp) = trade_data.trailing_percent { data.insert("trailingPercent".to_string(), tp); }
        if let Some(trig) = trade_data.trigger_price { data.insert("triggerPrice".to_string(), trig); }
        if order_type == OrderType::MARKET { data.insert("timeInForce".to_string(), "FOK".to_string()); }
        if let Some(red) = trade_data.reduce_only { data.insert("reduceOnly".to_string(), format!("{}", red)); }
        data.insert("signature".to_string(), order_signature);

        let json_data = serde_json::ser::to_string(&data).unwrap();

        let (signature, timestamp) = self.signature_for("/v3/orders".to_string(), "POST".to_string(), Some(json_data));

        let client = Client::new();
        let response = client.post(url)
            .header("dydx-signature".to_string(), signature)
            .header("dydx-api-key".to_string(), self.api_key.clone())
            .header("dydx-timestamp".to_string(), timestamp)
            .header("dydx-passphrase".to_string(), self.api_passphrase.clone())
            .json(&data)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
 
    pub async fn equity(&self, testnet: bool) -> anyhow::Result<f64> {
        let url = {
            if testnet { url::Url::parse("https://api.stage.dydx.exchange/v3/accounts").unwrap() }
            else { url::Url::parse("https://api.dydx.exchange/v3/accounts").unwrap() }
        };

        let (signature, timestamp) = self.signature_for("/v3/accounts".to_string(), "GET".to_string(), None);
        
        let client = Client::new();
        let response = client.get(url)
            .header("dydx-signature".to_string(), signature)
            .header("dydx-api-key".to_string(), self.api_key.clone())
            .header("dydx-timestamp".to_string(), timestamp)
            .header("dydx-passphrase".to_string(), self.api_passphrase.clone())
            .send()
            .await?
            .text()
            .await?;

        let parsed_response = &serde_json::from_str::<serde_json::Value>(&response).unwrap()["accounts"];
        Ok(parsed_response[0]["equity"].as_str().unwrap().parse::<f64>().unwrap())
    }
 
    pub async fn cancel_order(&self, order_id: String, testnet: bool) -> anyhow::Result<String> {

        let api_url = {
            if testnet { format!("https://api.stage.dydx.exchange/v3/orders/{}", order_id) }
            else { format!("https://api.dydx.exchange/v3/orders/{}", order_id) }
        };
        let url = url::Url::parse(&api_url).unwrap();

        let (signature, timestamp) = self.signature_for(format!("/v3/orders/{}", order_id), "DELETE".to_string(), None);
        
        let client = Client::new();
        let response = client.delete(url)
            .header("dydx-signature".to_string(), signature)
            .header("dydx-api-key".to_string(), self.api_key.clone())
            .header("dydx-timestamp".to_string(), timestamp)
            .header("dydx-passphrase".to_string(), self.api_passphrase.clone())
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    pub async fn find_position_id(&self, testnet: bool) -> anyhow::Result<u64> {

        let response: serde_json::Value = serde_json::from_str(&self.accounts(testnet).await.unwrap()).unwrap();
        let position_id = response["accounts"][0]["positionId"].as_str().unwrap().parse::<u64>().unwrap();
        Ok(position_id)
    }

}

