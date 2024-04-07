use ig_trading_api::streaming_api::StreamingApi;
use lightstreamer_client::item_update::ItemUpdate;
use lightstreamer_client::subscription::{Snapshot, Subscription, SubscriptionMode};
use lightstreamer_client::subscription_listener::SubscriptionListener;
use std::error::Error;

pub struct MySubscriptionListener {}

impl SubscriptionListener for MySubscriptionListener {
    fn on_item_update(&mut self, update: ItemUpdate) {
        println!(
            "UPDATE {} {}",
            update.get_value("stock_name").unwrap(),
            update.get_value("last_price").unwrap()
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //
    // Create a new subscription instance.
    //
    let mut my_subscription = Subscription::new(
        // Subscription mode.
        SubscriptionMode::Merge,
        // Subscription items, i.e. instruments to subscribe to.
        Some(vec![
            //"MARKET:IX.D.DAX.IFMM.IP".to_string(), // DAX40 Cash 1€
            "MARKET:CS.D.BITCOIN.CFD.IP".to_string(), // Bitcoin
        ]),
        // Subscription fields, i.e. data fields to receive updates for.
        Some(vec![
            "BID".to_string(),
            "OFFER".to_string(),
            "HIGH".to_string(),
            "LOW".to_string(),
            "MID_OPEN".to_string(),
            "CHANGE".to_string(),
            "CHANGE_PCT".to_string(),
            "MARKET_DELAY".to_string(),
            "MARKET_STATE".to_string(),
            "UPDATE_TIME".to_string(),
        ]),
    )?;
    my_subscription.set_requested_snapshot(Some(Snapshot::Yes))?;
    my_subscription.add_listener(Box::new(MySubscriptionListener {}));

    let mut streaming_api = StreamingApi::new(vec![my_subscription]).await?;
    streaming_api.connect().await;

    Ok(())
}
