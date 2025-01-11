use crate::common::*;
use crate::rest_regex::*;
use chrono::{NaiveDateTime, Utc};
use serde::de::DeserializeOwned;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::error::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// TRAITS FOR MODEL VALIDATION.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

///Trait to validate the fields of a request before sending it to the REST API.
pub trait ValidateRequest: Serialize {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

/// Trait to validate the fields of a response coming from the REST API.
pub trait ValidateResponse: DeserializeOwned {
    /// Improved deserialization function that provides better error messages using serde_path_to_error.
    fn deserialize<'de, T>(value: &'de Value) -> Result<T, Box<dyn Error>>
    where
        T: Deserialize<'de>,
    {
        let result = serde_path_to_error::deserialize(value);

        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(Box::new(ApiError {
                message: format!("Failed to deserialize JSON serde_json::Value: {}", e),
            })),
        }
    }

    fn from_value(value: &Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let instance: Self = <Self as ValidateResponse>::deserialize(value)?;

        match instance.validate() {
            Ok(()) => Ok(instance),
            Err(e) => Err(Box::new(ApiError {
                message: format!("Validation failed: {}", e),
            })),
        }
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

/// Struct to represent an empty object for use in optional parameters that
/// must implement Serialize and ValidateRequest traits.
#[derive(Serialize)]
pub struct Empty {}

impl ValidateRequest for Empty {}
impl ValidateRequest for &Empty {}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// REST API MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct SentimentQuery {
    pub market_ids: Option<Vec<String>>,
}

impl Serialize for SentimentQuery {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("SentimentQuery", 1)?;

        match self.market_ids.as_ref() {
            Some(ids) => {
                state.serialize_field("marketIds", &ids.join(","))?;
            }
            None => {
                state.serialize_field("marketIds", &None::<()>)?;
            }
        }

        state.end()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiments {
    pub client_sentiments: Vec<Sentiment>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiment {
    pub long_position_percentage: f64,
    pub market_id: String,
    pub short_position_percentage: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub allow_equities: bool,
    pub allow_quote_orders: bool,
    pub allowance_account_historical_data: f64,
    pub allowance_account_overall: f64,
    pub allowance_account_trading: f64,
    pub allowance_application_overall: f64,
    pub api_key: String,
    pub concurrent_subscriptions_limit: f64,
    pub created_date: String,
    pub name: String,
    pub status: ApplicationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationStatus {
    Disabled,
    Enabled,
    Revoked,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplication {
    pub allowance_account_overall: f64,
    pub allowance_account_trading: f64,
    pub api_key: String,
    pub status: ApplicationStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSearch {
    pub markets: Vec<MarketData>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PricesQuery {
    pub resolution: Option<Resolution>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub max: Option<u32>,
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Resolution {
    Day,
    Hour,
    Hour2,
    Hour3,
    Hour4,
    Minute,
    Minute10,
    Minute15,
    Minute2,
    Minute3,
    Minute30,
    Minute5,
    Month,
    Second,
    Week,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub instrument_type: InstrumentType,
    pub metadata: PriceMetadata,
    pub prices: Vec<Price>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceMetadata {
    pub page_data: PricePaging,
    pub size: f64,
    pub allowance: PriceAllowance,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricePaging {
    pub page_number: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAllowance {
    pub allowance_expiry: u32,
    pub remaining_allowance: u32,
    pub total_allowance: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub close_price: AskBid,
    pub high_price: AskBid,
    pub last_traded_volume: f64,
    pub low_price: AskBid,
    pub open_price: AskBid,
    pub snapshot_time: String,
    #[serde(rename = "snapshotTimeUTC")]
    pub snapshot_time_utc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskBid {
    pub ask: f64,
    pub bid: f64,
    pub last_traded: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlists {
    pub watchlists: Vec<Watchlist>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlist {
    pub default_system_watchlist: bool,
    pub deleteable: bool,
    pub editable: bool,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchlist {
    pub epics: Vec<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchlistResult {
    pub status: CreateWatchlistStatus,
    pub watchlist_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateWatchlistStatus {
    Success,
    SuccesNotAllInstrumentsAdded,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToWatchlist {
    pub epic: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// ACCOUNT ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Account data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Account alias.
    pub account_alias: Option<String>,
    /// Account identifier.
    pub account_id: String,
    /// Account name.
    pub account_name: String,
    /// Account type.
    pub account_type: AccountType,
    /// Account balances.
    pub balance: Balance,
    /// True if account can be transferred to.
    pub can_transfer_from: bool,
    /// True if account can be transferred from.
    pub can_transfer_to: bool,
    /// Account currency.
    pub currency: String,
    /// True if this the default login account.
    pub preferred: bool,
    /// Account status.
    pub status: AccountStatus,
}

/// Account status.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    /// Disabled account.
    Disabled,
    /// Enabled account.
    Enabled,
    /// Account is suspended from dealing.
    SuspendedFromDealing,
}

/// Account type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// CFD account.
    Cfd,
    /// Physical account.
    Physical,
    /// Spread bet account.
    Spreadbet,
}

/// Response to the GET /accounts request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsGetResponse {
    /// List of accounts.
    pub accounts: Vec<Account>,
}

impl ValidateResponse for AccountsGetResponse {}

/// Edits the account preferences by sending a PUT request to
/// the /accounts/preferences endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsPreferencesPutRequest {
    /// New trailing stop preference.
    pub trailing_stops_enabled: bool,
}

impl ValidateRequest for AccountsPreferencesPutRequest {}

/// Returns all account related preferences. Response to the GET /accounts/preferences request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsPreferencesGetResponse {
    /// New trailing stop preference.
    pub trailing_stops_enabled: bool,
}

impl ValidateResponse for AccountsPreferencesGetResponse {}

/// Returns the outcome of the account settings edit operation. Response to the
/// PUT /accounts/preferences request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsPreferencesStatusPutResponse {
    /// Status of the request.
    pub status: AccountsPreferencesPutRequestStatus,
}

impl ValidateResponse for AccountsPreferencesStatusPutResponse {}

/// Account balances.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Amount available for trading.
    pub available: f64,
    /// Balance of funds in the account.
    pub balance: f64,
    /// Minimum deposit amount required for margins.
    pub deposit: f64,
    /// Profit and loss amount.
    pub profit_loss: f64,
}

/// Status of the request. There is currently only one value but the list may be expanded in future.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountsPreferencesPutRequestStatus {
    Success,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// CONFIRMS ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Affected deal.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedDeal {
    /// Deal identifier.
    pub deal_id: String,
    /// Deal status.
    pub status: AffectedDealStatus,
}

/// Affected deal status.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AffectedDealStatus {
    /// Amended.
    Amended,
    /// Deleted.
    Deleted,
    /// Fully closed.
    FullyClosed,
    /// Opened.
    Opened,
    /// Partially closed.
    PartiallyClosed,
}

/// GET Request to /confirms/{dealReference} endpoint to retrieve deal confirmations for
/// the given deal reference. Please note, this should only be used if the deal confirmation
/// isn't received via the streaming API.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmsGetRequest {
    pub deal_reference: String,
}

impl ValidateRequest for ConfirmsGetRequest {}

/// Response to the GET /confirms request (Deal confirmation)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmsGetResponse {
    /// Affected deals.
    pub affected_deals: Vec<AffectedDeal>,
    /// Transaction date.
    pub date: String,
    /// Deal identifier.
    pub deal_id: String,
    /// Deal reference.
    pub deal_reference: String,
    /// Deal status.
    pub deal_status: DealStatus,
    /// Deal direction.
    pub direction: Direction,
    /// Instrument epic identifier.
    pub epic: String,
    /// Instrument expiry.
    pub expiry: Option<String>,
    /// True if guaranteed stop.
    pub guaranteed_stop: bool,
    /// Level at which the deal was opened.
    pub level: Option<f64>,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Profit.
    pub profit: Option<f64>,
    /// Profit currency.
    pub profit_currency: Option<String>,
    /// Describes the error (or success) condition for the specified trading operation.
    pub reason: DealReason,
    /// Size of the deal.
    pub size: Option<f64>,
    /// Position status.
    pub status: Option<PositionStatus>,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// True if trailing stop.
    pub trailing_stop: bool,
}

impl ValidateResponse for ConfirmsGetResponse {}

/// Describes the error (or success) condition for the specified trading operation.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DealReason {
    /// The account is not enabled to trade.
    AccountNotEnabledToTrading,
    /// The level of the attached stop or limit is not valid.
    AttachedOrderLevelError,
    /// The trailing stop value is invalid.
    AttachedOrderTrailingStopError,
    /// Cannot change the stop type.
    CannotChangeStopType,
    /// Cannot remove the stop.
    CannotRemoveStop,
    /// We are not taking opening deals on a Controlled Risk basis on this market.
    ClosingOnlyTradesAcceptedOnThisMarket,
    /// You are currently restricted from opening any new positions on your account.
    ClosingsOnlyAccount,
    /// Resubmitted request does not match the original order.
    ConflictingOrder,
    /// Instrument has an error - check the order's currency is the instrument's currency
    /// (see the market's details); otherwise please contact support.
    ContactSupportInstrumentError,
    /// Sorry we are unable to process this order. The stop or limit level you have requestedi
    /// is not a valid trading level in the underlying market.
    CrSpacing,
    /// The order has been rejected as it is a duplicate of a previously issued order.
    DuplicateOrderError,
    /// Exchange check failed. Please call in for assistance.
    ExchangeManualOverride,
    /// Order expiry is less than the sprint market's minimum expiry. Check the sprint market's
    /// market details for the allowable expiries.
    ExpiryLessThanSprintMarketMinExpiry,
    /// The total size of deals placed on this market in a short period has exceeded our limits.
    /// Please wait before attempting to open further positions on this market.
    FinanceRepeatDealing,
    /// Ability to force open in different currencies on same market not allowed.
    ForceOpenOnSameMarketDifferentCurrency,
    /// An error has occurred but no detailed information is available. Check transaction
    /// history or contact support for further information.
    GeneralError,
    /// The working order has been set to expire on a past date.
    GoodTillDateInThePast,
    /// The requested market was not found.
    InstrumentNotFound,
    /// Instrument not tradeable in this currency.
    InstrumentNotTradeableInThisCurrency,
    /// The account has not enough funds available for the requested trade.
    InsufficientFunds,
    /// The market level has moved and has been rejected.
    LevelToleranceError,
    /// The deal has been rejected because the limit level is inconsistent with current market
    /// price given the direction.
    LimitOrderWrongSideOfMarket,
    /// The manual order timeout limit has been reached.
    ManualOrderTimeout,
    /// Order declined during margin checks Check available funds.
    MarginError,
    /// The market is currently closed.
    MarketClosed,
    /// The market is currently closed with edits.
    MarketClosedWithEdits,
    /// The epic is due to expire shortly, client should deal in the next available contract.
    MarketClosing,
    /// The market does not allow opening shorting positions.
    MarketNotBorrowable,
    /// The market is currently offline.
    MarketOffline,
    /// The epic does not support 'Market' order type.
    MarketOrdersNotAllowedOnInstrument,
    /// The market can only be traded over the phone.
    MarketPhoneOnly,
    /// The market has been rolled to the next period.
    MarketRolled,
    /// The requested market is not allowed to this account.
    MarketUnavailableToClient,
    /// The order size exceeds the instrument's maximum configured value for auto-hedging
    /// the exposure of a deal.
    MaxAutoSizeExceeded,
    /// The order size is too small.
    MinimumOrderSizeError,
    /// The limit level you have requested is closer to the market level than the existing
    /// stop. When the market is closed you can only move the limit order further away from
    /// the current market level.
    MoveAwayOnlyLimit,
    /// The stop level you have requested is closer to the market level than the existing
    /// stop level. When the market is closed you can only move the stop level further away
    /// from the current market level.
    MoveAwayOnlyStop,
    /// The order level you have requested is moving closer to the market level than the
    /// exisiting order level. When the market is closed you can only move the order further
    /// away from the current market level.
    MoveAwayOnlyTriggerLevel,
    /// You are not permitted to open a non-controlled risk position on this account.
    NcrPositionsOnCrAccount,
    /// Opening CR position in opposite direction to existing CR position not allowed.
    OpposingDirectionOrdersNotAllowed,
    /// The deal has been rejected to avoid having long and short open positions on the same
    /// market or having long and short open positions and working orders on the same epic.
    OpposingPositionsNotAllowed,
    /// Order declined; please contact Support.
    OrderDeclined,
    /// The order is locked and cannot be edited by the user.
    OrderLocked,
    /// The order has not been found.
    OrderNotFound,
    /// The order size cannot be filled at this price at the moment.
    OrderSizeCannotBeFilled,
    /// The total position size at this stop level is greater than the size allowed on this market. Please reduce the size of the order.
    OverNormalMarketSize,
    /// Position cannot be deleted as it has been partially closed.
    PartialyClosedPositionNotDeleted,
    /// The deal has been rejected because of an existing position. Either set the 'force open' to be true or cancel opposing position.
    PositionAlreadyExistsInOppositeDirection,
    /// Position cannot be cancelled. Check transaction history or contact support for further information.
    PositionNotAvailableToCancel,
    /// Cannot close this position. Either the position no longer exists, or the size available to close is less than the size specified.
    PositionNotAvailableToClose,
    /// The position has not been found.
    PositionNotFound,
    /// Invalid attempt to submit a CFD trade on a spreadbet account.
    RejectCfdOrderOnSpreadbetAccount,
    /// Invalid attempt to submit a spreadbet trade on a CFD account.
    RejectSpreadbetOrderOnCfdAccount,
    /// Order size is not an increment of the value specified for the market.
    SizeIncrement,
    /// The expiry of the position would have fallen after the closing time of the market.
    SprintMarketExpiryAfterMarketClose,
    /// The market does not allow stop or limit attached orders.
    StopOrLimitNotAllowed,
    /// The order requires a stop.
    StopRequiredError,
    /// The submitted strike level is invalid.
    StrikeLevelTolerance,
    /// The operation completed successfully.
    Success,
    /// The market or the account do not allow for trailing stops.
    TrailingStopNotAllowed,
    /// The operation resulted in an unknown result condition. Check transaction history or contact support for further information.
    Unknown,
    /// The requested operation has been attempted on the wrong direction.
    WrongSideOfMarket,
}

/// Deal status.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DealStatus {
    /// Accepted.
    Accepted,
    /// Rejected.
    Rejected,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// HISTORY ENDPOINT MODELS (ACTIVITY).
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// The channel which triggered the activity.
    pub channel: ActivityChannel,
    /// The date of the activity item.
    pub date: String,
    /// Deal identifier.
    pub deal_id: String,
    /// Activity description.
    pub description: String,
    /// Activity details.
    pub details: Option<ActivityDetails>,
    /// Instrument epic identifier.
    pub epic: String,
    /// The period of the activity item, e.g. "DFB" or "02-SEP-11".
    /// This will be the expiry time/date for sprint markets,
    /// e.g. "2015-10-13T12:42:05"
    pub period: String,
    /// Activity status.
    pub status: ActivityStatus,
    /// Activity type.
    pub r#type: ActivityType,
}

/// Deal affected by an activity.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAction {
    /// Action type.
    pub action_type: ActivityActionType,
    /// Affected deal ID.
    pub affected_deal_id: String,
}

/// Activity action type.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityActionType {
    /// Limit order amended.
    LimitOrderAmended,
    /// Limit order deleted.
    LimitOrderDeleted,
    /// Limit order filled.
    LimitOrderFilled,
    /// Limit order opened.
    LimitOrderOpened,
    /// Limit order rolled.
    LimitOrderRolled,
    /// Position amended.
    PositionClosed,
    /// Position deleted.
    PositionDeleted,
    /// Position opened.
    PositionOpened,
    /// Position partially closed.
    PositionPartiallyClosed,
    /// Position rolled.
    PositionRolled,
    /// Stop order amended.
    StopLimitAmended,
    /// Stop order deleted.
    StopOrderAmended,
    /// Stop order filled.
    StopOrderDeleted,
    /// Stop order opened.
    StopOrderFilled,
    /// Stop order rolled.
    StopOrderOpened,
    /// Stop order rolled.
    StopOrderRolled,
    /// Unknown.
    Unknown,
    /// Working order amended.
    WorkingOrderDeleted,
}

/// The channel which triggered the activity.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityChannel {
    /// Dealer.
    Dealer,
    /// Mobile.
    Mobile,
    /// Public FIX API.
    PublicFixApi,
    /// Public Web API.
    PublicWebApi,
    /// System.
    System,
    /// Web.
    Web,
}

/// Activity details.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDetails {
    /// Deal affected by an activity.
    pub actions: Vec<ActivityAction>,
    /// Currency.
    pub currency: String,
    /// Deal reference.
    pub deal_reference: String,
    /// Deal direction.
    pub direction: Direction,
    /// Good till date.
    pub good_till_date: String,
    /// Guaranteed stop.
    pub guaranteed_stop: bool,
    /// Level.
    pub level: f64,
    /// Limit distance.
    pub limit_distance: f64,
    /// Limit level.
    pub limit_level: f64,
    /// Market name.
    pub market_name: String,
    /// Size.
    pub size: f64,
    /// Stop distance.
    pub stop_distance: f64,
    /// Stop level.
    pub stop_level: f64,
    /// Trailing step size.
    pub trailing_step: f64,
    /// Trailing stop distance.
    pub trailing_stop_distance: f64,
}

/// Returns the activity history by sending a GET request to the /history/activity endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoryGetRequest {
    /// Start date.
    pub from: NaiveDateTime,
    /// End date (Default = current time. A date without time
    /// component refers to the end of that day.).
    pub to: Option<NaiveDateTime>,
    /// Indicates whether to retrieve additional details about the activity.
    pub detailed: Option<bool>,
    /// Deal ID.
    pub deal_id: Option<String>,
    /// FIQL filter (supported operators: ==|!=|,|;).
    pub filter: Option<String>,
    /// Page size (min: 10, max: 500).
    pub page_size: Option<u32>,
}

/// Implement the ValidateRequest trait for the ActivityHistoryGetRequest struct.
impl ValidateRequest for ActivityHistoryGetRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Check if the 'from' date is not greater than today.
        if self.from > Utc::now().naive_utc() {
            return Err(Box::new(ApiError {
                message: "'From' date cannot be greater than today.".to_string(),
            }));
        }

        // Check if the 'from' date is not greater than 'to'.
        if let Some(to) = self.to {
            if self.from > to {
                return Err(Box::new(ApiError {
                    message: "'From' date cannot be greater than 'to' date.".to_string(),
                }));
            }
        }

        Ok(())
    }
}

/// Response to the GET /history/activity request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoryGetResponse {
    pub activities: Vec<Activity>,
    pub metadata: ActivityMetadata,
}

impl ValidateResponse for ActivityHistoryGetResponse {}

/// Paging metadata.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMetadata {
    /// Paging metadata.
    pub paging: ActivityPaging,
}

/// Paging metadata.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPaging {
    /// Next page.
    pub next: Option<String>,
    /// Page size.
    pub size: u32,
}

/// Activity status.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityStatus {
    /// Accepted.
    Accepted,
    /// Rejected.
    Rejected,
    /// Unknown.
    Unknown,
}

/// Activity type.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    /// Amend stop or limit activity.
    EditStopAndLimit,
    /// Position activity.
    Position,
    /// System generated activity.
    System,
    /// Working order activity.
    WorkingOrder,
}

/// Deal direction.
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    /// Buy.
    #[default]
    Buy,
    /// Sell.
    Sell,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// HISTORY ENDPOINT MODELS (TRANSACTIONS).
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Transaction data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// True if this was a cash transaction.
    pub cash_transaction: bool,
    /// Level at which the order was closed.
    pub close_level: String,
    /// Order currency.
    pub currency: String,
    /// Local date.
    pub date: String,
    /// UTC date.
    pub date_utc: String,
    /// Instrument name.
    pub instrument_name: String,
    /// Position opened date.
    pub open_date_utc: String,
    /// Level at which the order was opened.
    pub open_level: String,
    /// Period.
    pub period: String,
    /// Profit and loss.
    pub profit_and_loss: String,
    /// Reference.
    pub reference: String,
    /// Formatted order size, including the direction (+ for buy, - for sell)
    pub size: String,
    /// Transaction type.
    pub transaction_type: String,
}

/// Returns the transaction history by sending a GET request to the /history/transactions endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryGetRequest {
    /// Transaction type.
    pub r#type: Option<TransactionType>,
    /// Start date.
    pub from: NaiveDateTime,
    /// End date (date without time refers to the end of that day).
    pub to: Option<NaiveDateTime>,
    /// Limits the timespan in seconds through to current time
    /// (not applicable if a date range has been specified).
    pub max_span_seconds: Option<u64>,
    /// Page size (disable paging = 0).
    pub page_size: Option<u32>,
    /// Page number.
    pub page_number: Option<u32>,
}

/// Implement the ValidateRequest trait for the TransactionHistoryGetRequest struct.
impl ValidateRequest for TransactionHistoryGetRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Check if the 'from' date is not greater than today.
        if self.from > Utc::now().naive_utc() {
            return Err(Box::new(ApiError {
                message: "'From' date cannot be greater than today.".to_string(),
            }));
        }

        // Check if the 'from' date is not greater than 'to'.
        if let Some(to) = self.to {
            if self.from > to {
                return Err(Box::new(ApiError {
                    message: "'From' date cannot be greater than 'to' date.".to_string(),
                }));
            }
        }

        Ok(())
    }
}

/// List of transactions. Response to the GET /history/transactions request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryGetResponse {
    /// Paging metadata.
    pub metadata: TransactionMetadata,
    /// Transaction data.
    pub transactions: Vec<Transaction>,
}

impl ValidateResponse for TransactionHistoryGetResponse {}

/// Paging metadata.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetadata {
    /// Paging metadata.
    pub page_data: TransactionPageData,
    /// Size.
    pub size: u32,
}

/// Paging metadata.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPageData {
    /// Page number.
    pub page_number: u32,
    /// Page size.
    pub page_size: u32,
    /// Total number of pages.
    pub total_pages: u32,
}

/// Transaction type.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// All.
    All,
    /// Deals.
    AllDeal,
    /// Deposit.
    Deposit,
    /// Withdrawal.
    Withdrawal,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// MARKETS ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Currency.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    /// Base exchange rate.
    pub base_exchange_rate: f64,
    /// Code, to be used when placing orders.
    pub code: String,
    /// Exchange rate.
    pub exchange_rate: f64,
    /// True if this is the default currency.
    pub is_default: bool,
    /// Symbol, for display purposes.
    pub symbol: String,
}

/// Dealing rule.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DealingRule {
    /// Describes the dimension for a dealing rule value.
    pub unit: RuleUnit,
    /// Value.
    pub value: f64,
}

/// Dealing rules.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DealingRules {
    /// Controlled risk spacing.
    pub controlled_risk_spacing: DealingRule,
    /// Client's market order trading preference
    pub market_order_preference: MarketOrderPreference,
    /// Max stop or limit distance.
    pub max_stop_or_limit_distance: DealingRule,
    /// Min controlled risk stop distance.
    pub min_controlled_risk_stop_distance: DealingRule,
    /// Min deal size.
    pub min_deal_size: DealingRule,
    /// Min normal stop or limit distance.
    pub min_normal_stop_or_limit_distance: DealingRule,
    /// Min step distance.
    pub min_step_distance: DealingRule,
    /// Trailing stops trading preference for the specified market.
    pub trailing_stops_preference: TrailingStopsPreference,
}

/// Deposit band.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositBand {
    /// The currency for this currency band factor calculation.
    pub currency: String,
    /// Margin Percentage.
    pub margin: f64,
    /// Band maximum.
    pub max: Option<f64>,
    /// Band minimum.
    pub min: f64,
}

/// Market expiry details.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expiry {
    /// Last dealing date.
    pub last_dealing_date: String,
    /// Settlement information.
    pub settlement_info: String,
}

/// Instrument details.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDetails {
    /// Chart code.
    pub chart_code: String,
    /// Contract size.
    pub contract_size: String,
    /// True if controlled risk trades are allowed.
    pub controlled_risk_allowed: bool,
    /// Country.
    pub country: Option<String>,
    /// Currencies.
    pub currencies: Vec<Currency>,
    /// Instrument identifier.
    pub epic: String,
    /// Expiry.
    pub expiry: String,
    /// Market expiry details.
    pub expiry_details: Option<Expiry>,
    /// True if force open is allowed.
    pub force_open_allowed: bool,
    /// The limited risk premium.
    pub limited_risk_premium: DealingRule,
    /// Lot size.
    pub lot_size: f64,
    /// Margin deposit bands.
    pub margin_deposit_bands: Vec<DepositBand>,
    /// Margin requirement factor.
    pub margin_factor: f64,
    /// Describes the dimension for a dealing rule value.
    pub margin_factor_unit: RuleUnit,
    /// Market identifier.
    pub market_id: String,
    /// Market name.
    pub name: String,
    /// Reuters news code.
    pub news_code: String,
    /// Meaning of one pip.
    pub one_pip_means: String,
    /// Market open and close times.
    pub opening_hours: Option<OpeningHours>,
    /// Market rollover details.
    pub rollover_details: Option<Rollover>,
    /// Slippage factor details for a given market.
    pub slippage_factor: SlippageFactor,
    /// List of special information notices.
    pub special_info: Vec<String>,
    /// For sprint markets only, the maximum value to be specified
    /// as the expiry of a sprint markets trade.
    pub sprint_markets_maximum_expiry_time: Option<f64>,
    /// For sprint markets only, the minimum value to be specified
    /// as the expiry of a sprint markets trade.
    pub sprint_markets_minimum_expiry_time: Option<f64>,
    /// True if stops and limits are allowed.
    pub stops_limits_allowed: bool,
    /// True if streaming prices are available, i.e. the market is
    /// open and the client has appropriate permissions.
    pub streaming_prices_available: bool,
    /// Instrument type.
    pub r#type: InstrumentType,
    /// Unit used to qualify the size of a trade.
    pub unit: InstrumentUnit,
    /// Value of one pip.
    pub value_of_one_pip: String,
}

/// Unit used to qualify the size of a trade.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentUnit {
    /// Amount.
    Amount,
    /// Contracts.
    Contracts,
    /// Shares.
    Shares,
}

/// Market details.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDetails {
    /// Dealing rules.
    pub dealing_rules: DealingRules,
    /// Instrument details.
    pub instrument: InstrumentDetails,
    /// Market snapshot data.
    pub snapshot: MarketSnapshot,
}

/// Filter for the market details.
#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketDetailsFilterType {
    /// Display all market details. Market details includes all instrument data,
    /// dealing rules and market snapshot values for all epics specified.
    All,
    /// Display the market snapshot and minimal instrument data fields.
    /// This mode is faster because it only sets the epic and instrument type in
    /// the instrument data and the market data snapshot values with all the other
    /// fields being unset for each epic specified.
    SnapshotOnly,
}

/// Market navigation data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketNavigationGetRequest {
    /// The identifier of the node to browse.
    node_id: Option<String>,
}

impl ValidateRequest for MarketNavigationGetRequest {}

/// Response to the GET /marketnavigation request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketNavigationGetResponse {
    /// Market data.
    pub markets: Option<Vec<MarketData>>,
    /// Market Hierarchy Nodes.
    pub nodes: Option<Vec<MarketNode>>,
}

impl ValidateResponse for MarketNavigationGetResponse {}

/// Market Hierarchy Node.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketNode {
    /// Node identifier.
    pub id: String,
    /// Node name.
    pub name: String,
}

/// Client's market order trading preference.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketOrderPreference {
    /// Market orders are allowed for the account type and instrument, and the user has
    /// enabled market orders in their preferences but decided the default state is off.
    AvailableDefaultOff,
    /// Market orders are allowed for the account type and instrument, and the user has
    /// enabled market orders in their preferences and has decided the default state is on.
    AvailableDefaultOn,
    /// Market orders are not allowed for the current site and/or instrument.
    NotAvailable,
}

/// Request to the GET /markets endpoint.
#[derive(Debug, Default)]
pub struct MarketsGetRequest {
    /// The epics of the market to be retrieved, separated by a comma.
    pub epics: Vec<String>,
    /// Filter for the market details.
    pub filter: Option<MarketDetailsFilterType>,
}

/// Implement the Serialize trait for the MarketsGetRequest struct.
impl Serialize for MarketsGetRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("MarketsQuery", 2)?;

        state.serialize_field("epics", &self.epics.join(","))?;

        match self.filter.as_ref() {
            Some(filter) => {
                state.serialize_field("filter", filter)?;
            }
            None => {
                state.serialize_field("marketIds", &None::<()>)?;
            }
        }

        state.end()
    }
}

/// Implement the ValidateRequest trait for the MarketsGetRequest struct.
impl ValidateRequest for MarketsGetRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: Size(min=1).
        if self.epics.is_empty() {
            return Err(Box::new(ApiError {
                message: "The 'epics' field cannot be empty.".to_string(),
            }));
        }

        // Constraint: Size(max=50).
        if self.epics.len() > 50 {
            return Err(Box::new(ApiError {
                message: "The 'epics' field cannot be greater than 50.".to_string(),
            }));
        }

        // Constraint: Pattern(regexp="^([A-Z]+(?:\.[A-Z]+)*(?:,[A-Z]+(?:\.[A-Z]+)*)*)$").
        let serialized_epics = self.epics.join(",");
        if !EPICS_REGEX.is_match(&serialized_epics) {
            return Err(Box::new(ApiError {
                message: format!("Epics field is invalid. Fields: {}", serialized_epics),
            }));
        }

        Ok(())
    }
}

/// Response to the GET /markets request.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsGetResponse {
    /// Market details.
    pub market_details: Vec<MarketDetails>,
}

impl ValidateResponse for MarketsGetResponse {}

/// Describes the dimension for a dealing rule value.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleUnit {
    /// Percentage.
    Percentage,
    /// Points.
    Points,
}

/// Market snapshot data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSnapshot {
    /// Bid price.
    pub bid: f64,
    /// Binary odds.
    pub binary_odds: Option<f64>,
    /// The number of points to add on each side of the market as an
    /// additional spread when placing a guaranteed stop trade.
    pub controlled_risk_extra_spread: f64,
    /// Number of decimal positions for market levels.
    pub decimal_places_factor: f64,
    /// Price delay.
    pub delay_time: f64,
    /// Highest price on the day.
    pub high: f64,
    /// Lowest price on the day.
    pub low: f64,
    /// Describes the current status of a given market.
    pub market_status: MarketStatus,
    /// Net price change on the day.
    pub net_change: f64,
    /// Offer price.
    pub offer: f64,
    /// Percentage price change on the day.
    pub percentage_change: f64,
    /// Multiplying factor to determine actual pip value for the
    /// levels used by the instrument.
    pub scaling_factor: f64,
    /// Time of last price update.
    pub update_time: String,
}

/// Market time range.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTime {
    /// Close time.
    pub close_time: String,
    /// Open time.
    pub open_time: String,
}

/// Market open and close times.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    /// Market time ranges.
    pub market_times: Vec<MarketTime>,
}

/// Market rollover details.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rollover {
    /// Last rollover time.
    pub last_rollover_time: String,
    /// Rollover info.
    pub rollover_info: String,
}

/// Slippage factor details for a given market.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SlippageFactor {
    /// Unit.
    pub unit: String,
    /// Value.
    pub value: f64,
}

/// Trailing stops trading preference for the specified market.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrailingStopsPreference {
    /// Trailing stops are allowed for the current market.
    Available,
    /// Trailing stops are not allowed for the current market.
    NotAvailable,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// POSITIONS ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Market data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    /// Bid.
    pub bid: Option<f64>,
    /// Instrument price delay (minutes).
    pub delay_time: f64,
    /// Instrument epic identifier.
    pub epic: String,
    /// Instrument expiry period.
    pub expiry: String,
    /// High price.
    pub high: Option<f64>,
    /// Instrument name.
    pub instrument_name: String,
    /// Instrument type.
    pub instrument_type: InstrumentType,
    /// Instrument lot size.
    pub lot_size: Option<f64>,
    /// Low price.
    pub low: Option<f64>,
    /// Describes the current status of a given market.
    pub market_status: MarketStatus,
    /// Price net change.
    pub net_change: f64,
    /// Offer.
    pub offer: Option<f64>,
    /// Price percentage change.
    pub percentage_change: f64,
    /// Multiplying factor to determine actual pip value for the
    /// levels used by the instrument.
    pub scaling_factor: f64,
    /// True if streaming prices are available, i.e. the market is
    /// tradeable and the client has appropriate permissions.
    pub streaming_prices_available: bool,
    /// Local time of last instrument price update.
    pub update_time: String,
    /// UTC time of last instrument price update.
    #[serde(rename = "updateTimeUTC")]
    pub update_time_utc: String,
}

/// Describes the current status of a given market.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketStatus {
    // Closed.
    Closed,
    /// Open for edits.
    EditsOnly,
    /// Offline.
    Offline,
    /// In auction mode.
    OnAuction,
    /// In no-edits mode.
    OnAuctionNoEdits,
    /// Suspended.
    Suspended,
    /// Open for trades.
    Tradeable,
}

/// Instrument type.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    Binary,
    BungeeCapped,
    BungeeCommodities,
    BungeeCurrencies,
    BungeeIndices,
    Commodities,
    Currencies,
    Indices,
    KnockoutsCommodities,
    KnockoutsCurrencies,
    KnockoutsIndices,
    KnockoutsShares,
    OptCommodities,
    OptCurrencies,
    OptIndices,
    OptRates,
    OptShares,
    Rates,
    Sectors,
    Shares,
    SprintMarket,
    TestMarket,
    Unknown,
}

/// Describes the order level model to be used for a position operation.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    /// Limit orders get executed at the price seen by IG at the moment of booking a trade.
    /// A limit determines the level at which the order or the remainder of the order will be rejected.
    Limit,
    /// Market orders get executed at the price seen by the IG at the time of booking the trade.
    /// A level cannot be specified. Not applicable to BINARY instruments.
    #[default]
    Market,
    /// Quote orders get executed at the specified level. The level has to be accompanied by a valid
    /// quote id. This type is only available subject to agreement with IG.
    Quote,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionStatus {
    Amended,
    Closed,
    Deleted,
    Open,
    PartiallyClosed,
}

/// Request to close a position by sending a DELETE request to the /positions/otc endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDeleteRequest {
    /// Deal identifier.
    pub deal_id: Option<String>,
    /// Deal direction.
    pub direction: Option<Direction>,
    /// Instrument epic identifier.
    pub epic: Option<String>,
    /// Instrument expiry.
    pub expiry: Option<String>,
    /// Closing deal level.
    pub level: Option<f64>,
    /// Describes the order level model to be used for a position operation.
    pub order_type: Option<OrderType>,
    /// Lightstreamer price quote identifier.
    pub quote_id: Option<String>,
    /// Deal size.
    pub size: f64,
    /// The time in force determines the order fill strategy.
    pub time_in_force: Option<TimeInForce>,
}

/// Implements the validation of the PositionDeleteRequest.
impl ValidateRequest for PositionDeleteRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: Pattern(regexp=".{1,30}")
        if let Some(deal_id) = &self.deal_id {
            if !DEAL_ID_REGEX.is_match(deal_id) {
                return Err(Box::new(ApiError {
                    message: "Deal ID field is invalid.".to_string(),
                }));
            }
        }

        // Constraint: Pattern(regexp="[A-Za-z0-9._]{6,30}")
        if let Some(epic) = &self.epic {
            if !EPIC_REGEX.is_match(epic) {
                return Err(Box::new(ApiError {
                    message: "Epic field is invalid.".to_string(),
                }));
            }
        }

        // Constraint: Pattern(regexp="(\\d{2}-)?[A-Z]{3}-\\d{2}|-|DFB")
        if let Some(expiry) = &self.expiry {
            if !EXPIRY_REGEX.is_match(expiry) {
                return Err(Box::new(ApiError {
                    message: "Expiry field is invalid.".to_string(),
                }));
            }
        }

        // Constraint: check precision of size is not more than 12 decimal places.
        let size_str = format!("{}", self.size);
        let parts: Vec<&str> = size_str.split('.').collect();
        if parts.len() == 2 && parts[1].len() > 12 {
            return Err(Box::new(ApiError {
                message: "Size field has more thatn 12 decimal places.".to_string(),
            }));
        }

        // Constraint: if epic is defined, then set expiry.
        if self.epic.is_some() && self.expiry.is_none() {
            return Err(Box::new(ApiError {
                message: "Expiry field is required when epic is defined.".to_string(),
            }));
        }

        // Constraint: if order_type equals LIMIT, then DO NOT set quote_id.
        if self.order_type == Some(OrderType::Limit) && self.quote_id.is_some() {
            return Err(Box::new(ApiError {
                message: "Quote ID field cannot be set when order type is LIMIT.".to_string(),
            }));
        }

        // Constraint: if order_type equals LIMIT, then set level.
        if self.order_type == Some(OrderType::Limit) && self.level.is_none() {
            return Err(Box::new(ApiError {
                message: "Level field is required when order type is LIMIT.".to_string(),
            }));
        }

        // Constraint: if order_type equals MARKET, then DO NOT set level, quote_id.
        if self.order_type == Some(OrderType::Market)
            && (self.level.is_some() || self.quote_id.is_some())
        {
            return Err(Box::new(ApiError {
                message: "Level and quote ID fields cannot be set when order type is MARKET."
                    .to_string(),
            }));
        }

        // Constraint: if order_type equals QUOTE, then set level, quoteId.
        if self.order_type == Some(OrderType::Quote)
            && (self.level.is_none() || self.quote_id.is_none())
        {
            return Err(Box::new(ApiError {
                message: "Level and quote ID fields are required when order type is QUOTE."
                    .to_string(),
            }));
        }

        // Constraint: set only one of {deal_id, epic}.
        if self.deal_id.is_some() && self.epic.is_some() {
            return Err(Box::new(ApiError {
                message: "Set only one of {deal_id, epic}.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to position close request (DELETE /positions/otc).
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDeleteResponse {
    pub deal_reference: String,
}

impl ValidateResponse for PositionDeleteResponse {}

/// Request an open position for the active account by deal identifier by sending
/// a GET request to the /positions/{dealId} endpoint.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionGetRequest {
    /// Deal identifier.
    pub deal_id: String,
}

/// Implements the validation of the PositionGetRequest.
impl ValidateRequest for PositionGetRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !DEAL_ID_REGEX.is_match(&self.deal_id) {
            return Err(Box::new(ApiError {
                message: "Deal ID field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// List of all the positions for the active account. Response to the GET /positions request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsGetResponse {
    /// List of positions.
    pub positions: Vec<PositionGetResponse>,
}

impl ValidateResponse for PositionsGetResponse {}

/// Open position data. Response to the GET /positions/{dealId} request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionGetResponse {
    /// Market data.
    pub market: MarketData,
    /// Position data.
    pub position: PositionData,
}

impl ValidateResponse for PositionGetResponse {}

/// Request to open a new position by sending a POST request to the /positions/otc endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionPostRequest {
    /// Currency code.
    pub currency_code: String,
    /// Deal reference. A user-defined reference identifying the submission of the order.
    pub deal_reference: Option<String>,
    /// Deal direction.
    pub direction: Direction,
    /// Instrument epic identifier.
    pub epic: String,
    /// Instrument expiry.
    pub expiry: String,
    /// True if force open is required.
    pub force_open: bool,
    /// True if a guaranteed stop is required.
    pub guaranteed_stop: bool,
    /// Deal level.
    pub level: Option<f64>,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Describes the order level model to be used for a position operation.
    pub order_type: OrderType,
    /// Lightstreamer price quote identifier.
    pub quote_id: Option<String>,
    /// Deal size.
    pub size: f64,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// The time in force determines the order fill strategy.
    pub time_in_force: Option<TimeInForce>,
    /// Whether the stop has to be moved towards the current level in case of a favourable trade.
    pub trailing_stop: Option<bool>,
    /// Increment step in pips for the trailing stop.
    pub trailing_stop_increment: Option<f64>,
}

/// Implements the validation of the PositionPostRequest.
impl ValidateRequest for PositionPostRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: if limit_distance is set, then force_open must be true.
        if self.limit_distance.is_some() && self.force_open != true {
            return Err(Box::new(ApiError {
                message: "force_open field must be true when limit_distance is set.".to_string(),
            }));
        }

        // Constraint: if limit_level is set, then force_open must be true.
        if self.limit_level.is_some() && self.force_open != true {
            return Err(Box::new(ApiError {
                message: "force_open field must be true when limit_level is set.".to_string(),
            }));
        }

        // Constraint: if stop_distance is set, then force_open must be true.
        if self.stop_distance.is_some() && self.force_open != true {
            return Err(Box::new(ApiError {
                message: "force_open field must be true when stop_distance is set.".to_string(),
            }));
        }

        // Constraint: if stop_level is set, then force_open must be true.
        if self.stop_level.is_some() && self.force_open != true {
            return Err(Box::new(ApiError {
                message: "force_open field must be true when stop_level is set.".to_string(),
            }));
        }

        // Constraint: if guaranteed_stop equals true, then set only one of stop_level, stop_distance.
        if self.guaranteed_stop == true && self.stop_level.is_some() && self.stop_distance.is_some()
        {
            return Err(Box::new(ApiError {
                message: "Only one of stop_level or stop_distance can be set when guaranteed_stop is true.".to_string(),
            }));
        }

        // Constraint: if order_type equals LIMIT, then DO NOT set quote_id.
        if self.order_type == OrderType::Limit && self.quote_id.is_some() {
            return Err(Box::new(ApiError {
                message: "quote_id cannot be set when order_type is LIMIT.".to_string(),
            }));
        }

        // Constraint: if order_type equals LIMIT, then set level.
        if self.order_type == OrderType::Limit && self.level.is_none() {
            return Err(Box::new(ApiError {
                message: "level must be set when order_type is LIMIT.".to_string(),
            }));
        }

        // Constraint: if order_type equals MARKET, then DO NOT set level, quote_id.
        if self.order_type == OrderType::Market && (self.level.is_some() || self.quote_id.is_some())
        {
            return Err(Box::new(ApiError {
                message: "Neither level nor quote_id can be set when order_type is MARKET."
                    .to_string(),
            }));
        }

        // Constraint: if order_type equals QUOTE, then set level, quote_id.
        if self.order_type == OrderType::Quote && (self.level.is_none() || self.quote_id.is_none())
        {
            return Err(Box::new(ApiError {
                message: "Both level and quote_id must be set when order_type is QUOTE."
                    .to_string(),
            }));
        }

        // Constraint: if trailing_stop equals false, then DO NOT set trailing_stop_increment.
        if self.trailing_stop == Some(false) && self.trailing_stop_increment.is_some() {
            return Err(Box::new(ApiError {
                message: "trailing_stop_increment cannot be set when trailing_stop is false."
                    .to_string(),
            }));
        }

        // Constraint: if trailing_stop equals true, then DO NOT set stop_level.
        if self.trailing_stop == Some(true) && self.stop_level.is_some() {
            return Err(Box::new(ApiError {
                message: "stop_level cannot be set when trailing_stop is true.".to_string(),
            }));
        }

        // Constraint: if trailing_stop equals true, then guaranteed_stop must be false.
        if self.trailing_stop == Some(true) && self.guaranteed_stop != false {
            return Err(Box::new(ApiError {
                message: "guaranteed_stop must be false when trailing_stop is true.".to_string(),
            }));
        }

        // Constraint: if trailing_stop equals true, then set stop_distance, trailing_stop_increment.
        if self.trailing_stop == Some(true)
            && (self.stop_distance.is_none() || self.trailing_stop_increment.is_none())
        {
            return Err(Box::new(ApiError {
                message: "Both stop_distance and trailing_stop_increment must be set when trailing_stop is true.".to_string(),
            }));
        }

        // Constraint: set only one of limit_level, limit_distance.
        if self.limit_level.is_some() && self.limit_distance.is_some() {
            return Err(Box::new(ApiError {
                message: "Only one of limit_level or limit_distance can be set.".to_string(),
            }));
        }

        // Constraint: set only one of stop_level, stop_distance.
        if self.stop_level.is_some() && self.stop_distance.is_some() {
            return Err(Box::new(ApiError {
                message: "Only one of stop_level or stop_distance can be set.".to_string(),
            }));
        }

        // Constraint: field currency_code follows pattern(regexp="[A-Z]{3}").
        if !CURRENCY_CODE_REGEX.is_match(&self.currency_code) {
            return Err(Box::new(ApiError {
                message: "Currency code field is invalid.".to_string(),
            }));
        }

        // Constraint: field deal_reference follows pattern(regexp="[A-Za-z0-9_\\-]{1,30}")].
        if let Some(deal_reference) = &self.deal_reference {
            if !DEAL_REFERENCE_REGEX.is_match(deal_reference) {
                return Err(Box::new(ApiError {
                    message: "Deal reference field is invalid.".to_string(),
                }));
            }
        }

        // Constraint: field epic follows pattern(regexp="[A-Za-z0-9._]{6,30}").
        if !EPIC_REGEX.is_match(&self.epic) {
            return Err(Box::new(ApiError {
                message: "Epic field is invalid.".to_string(),
            }));
        }

        // Constraint: field expiry follows pattern(regexp="(\\d{2}-)?[A-Z]{3}-\\d{2}|-|DFB").
        if !EXPIRY_REGEX.is_match(&self.expiry) {
            return Err(Box::new(ApiError {
                message: "Expiry field is invalid.".to_string(),
            }));
        }

        // Constraint: check precision of size is not more than 12 decimal places.
        let size_str = format!("{}", self.size);
        let parts: Vec<&str> = size_str.split('.').collect();
        if parts.len() == 2 && parts[1].len() > 12 {
            return Err(Box::new(ApiError {
                message: "Size field has more thatn 12 decimal places.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to position open request (POST /positions/otc).
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionPostResponse {
    /// Deal reference of the transaction.
    pub deal_reference: String,
}

impl ValidateResponse for PositionPostResponse {}

/// Request to update a position by sending a PUT request to the /positions/otc/{deal_id} endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionPutRequest {
    /// True if a guaranteed stop is required.
    pub guaranteed_stop: Option<bool>,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// True if Trailing stop is required.
    pub trailing_stop: Option<bool>,
    ///	Trailing stop distance.
    pub trailing_stop_distance: Option<f64>,
    /// Trailing stop increment.
    pub trailing_stop_increment: Option<f64>,
}

/// Implement the ValidateRequest trait for PositionPutRequest.
impl ValidateRequest for PositionPutRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: if guaranteed_stop equals true, then set stop_level.
        if self.guaranteed_stop == Some(true) && self.stop_level.is_none() {
            return Err(Box::new(ApiError {
                message: "stop_level must be set when guaranteed_stop is true.".to_string(),
            }));
        }

        // Constraint: if guaranteed_stop equals true, then trailing_stop must be false.
        if self.guaranteed_stop == Some(true) && self.trailing_stop == Some(true) {
            return Err(Box::new(ApiError {
                message: "trailing_stop must be false when guaranteed_stop is true.".to_string(),
            }));
        }

        // Constraint: if trailing_stop equals false, then DO NOT set trailing_stop_distance, trailing_stop_increment.
        if self.trailing_stop == Some(false)
            && (self.trailing_stop_distance.is_some() || self.trailing_stop_increment.is_some())
        {
            return Err(Box::new(ApiError {
                message: "Neither trailing_stop_distance nor trailing_stop_increment can be set when trailing_stop is false.".to_string(),
            }));
        }

        // Constraint: if trailing_stop equals true, then guaranteed_stop must be false.
        if self.trailing_stop == Some(true) && self.guaranteed_stop == Some(true) {
            return Err(Box::new(ApiError {
                message: "guaranteed_stop must be false when trailing_stop is true.".to_string(),
            }));
        }

        // Constraint: if trailing_stop equals true, then set trailing_stop_distance, trailing_stop_increment, stop_level.
        if self.trailing_stop == Some(true)
            && (self.trailing_stop_distance.is_none()
                || self.trailing_stop_increment.is_none()
                || self.stop_level.is_none())
        {
            return Err(Box::new(ApiError {
                message: "All of trailing_stop_distance, trailing_stop_increment, stop_level must be set when trailing_stop is true.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to position update request (PUT /positions/otc/{deal_id}).
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionPutResponse {
    /// Deal reference.
    pub deal_reference: String,
}

impl ValidateResponse for PositionPutResponse {}

/// Position data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    /// Size of the contract.
    pub contract_size: f64,
    /// True if position is risk controlled.
    pub controlled_risk: bool,
    /// Local date the position was opened.
    pub created_date: String,
    /// UTC date the position was opened.
    #[serde(rename = "createdDateUTC")]
    pub created_date_utc: String,
    /// Position currency ISO code.
    pub currency: String,
    /// Deal identifier.
    pub deal_id: String,
    /// Deal reference.
    pub deal_reference: String,
    /// Deal direction.
    pub direction: Direction,
    /// Level at which the position was opened.
    pub level: f64,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Limited Risk Premium.
    pub limited_risk_premium: Option<f64>,
    /// Deal size.
    pub size: f64,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// Trailing step size.
    pub trailing_step: Option<f64>,
    /// Trailing stop distance.
    pub trailing_stop_distance: Option<f64>,
}

/// The time in force determines the order fill strategy.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// Execute and eliminate.
    ExecuteAndEliminate,
    /// Fill or kill.
    FillOrKill,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// POSITIONS SPRINTMARKETS ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Sprint market expiry period.
#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SprintMarketExpiryPeriod {
    // 5 minutes.
    FiveMinutes,
    // 1 minute.
    OneMinute,
    // 60 minutes.
    SixtyMinutes,
    // 20 minutes.
    TwentyMinutes,
    // 2 minutes.
    TwoMinutes,
}

/// Sprint market position data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintMarketPosition {
    /// Date the position was opened.
    pub created_date: String,
    /// Currency of the payout.
    pub currency: String,
    /// Deal identifier.
    pub deal_id: String,
    /// Description.
    pub description: String,
    /// Deal direction.
    pub direction: Direction,
    /// Instrument epic identifier.
    pub epic: String,
    /// Expiry time.
    pub expiry_time: String,
    /// Instrument name.
    pub instrument_name: String,
    /// Describes the current status of a given market.
    pub market_status: MarketStatus,
    /// Payout amount.
    pub payout_amount: f64,
    /// Size.
    pub size: f64,
    /// Strike price.
    pub strike_level: f64,
}

/// Request to get the sprint market positions by sending a GET request to the /positions/sprintmarkets endpoint.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintMarketPositionsGetResponse {
    /// List of sprint market positions.
    pub sprint_market_positions: Vec<SprintMarketPosition>,
}

/// Validate the sprint market positions response.
impl ValidateResponse for SprintMarketPositionsGetResponse {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        for sprint_market_position in &self.sprint_market_positions {
            // Constraint: field currency follows pattern(regexp="[A-Z]{3}").
            if !CURRENCY_CODE_REGEX.is_match(&sprint_market_position.currency) {
                return Err(Box::new(ApiError {
                    message: format!(
                        "Currency code '{}' field is invalid.",
                        sprint_market_position.currency
                    ),
                }));
            }
        }

        Ok(())
    }
}

/// Create sprint market position request. A request will be executed as a market order based on
/// the current trade odds and strike level. An indicative payout amount (payout = premium / odds)
/// can be evaluated by obtaining the binary odds ratio from the market details endpoint prior to
/// placing an order.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintMarketPositionsPostRequest {
    /// A user-defined reference identifying the submission of the order.
    pub deal_reference: Option<String>,
    /// Deal direction.
    pub direction: Option<Direction>,
    /// Instrument epic identifier.
    pub epic: String,
    /// Sprint market expiry period.
    pub expiry_period: Option<SprintMarketExpiryPeriod>,
    /// Deal size.
    pub size: f64,
}

/// Validate the sprint market position request.
impl ValidateRequest for SprintMarketPositionsPostRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: field deal_reference follows pattern(regexp="[A-Za-z0-9_\\-]{1,30}")].
        if let Some(deal_reference) = &self.deal_reference {
            if !DEAL_REFERENCE_REGEX.is_match(deal_reference) {
                return Err(Box::new(ApiError {
                    message: "Deal reference field is invalid.".to_string(),
                }));
            }
        }

        // Constraint: field epic follows pattern(regexp="[A-Za-z0-9._]{6,30}").
        if !EPIC_REGEX.is_match(&self.epic) {
            return Err(Box::new(ApiError {
                message: "Epic field is invalid.".to_string(),
            }));
        }

        // Constraint: check precision of size is not more than 12 decimal places.
        let size_str = format!("{}", self.size);
        let parts: Vec<&str> = size_str.split('.').collect();
        if parts.len() == 2 && parts[1].len() > 12 {
            return Err(Box::new(ApiError {
                message: "Size field has more thatn 12 decimal places.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to the create sprint market position request.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintMarketPositionsPostResponse {
    /// Deal reference of the transaction.
    pub deal_reference: String,
}

impl ValidateResponse for SprintMarketPositionsPostResponse {}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// SESSION ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Switch the active account by sending a PUT request to the /session endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSwitchPutRequest {
    pub account_id: String,
    pub default_account: Option<bool>,
}

/// Validate the account switch request.
impl ValidateRequest for AccountSwitchPutRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !ACCOUNT_ID_REGEX.is_match(&self.account_id) {
            return Err(Box::new(ApiError {
                message: "Account ID field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to the PUT /session request for account switching.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSwitchPutResponse {
    pub dealing_enabled: bool,
    pub has_active_demo_accounts: bool,
    pub has_active_live_accounts: bool,
    pub trailing_stops_enabled: bool,
}

/// Validate the account switch response.
impl ValidateResponse for AccountSwitchPutResponse {}

/// Authenticate the user by sending a POST request to the /session endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationPostRequest {
    pub identifier: String,
    pub password: String,
}

/// Validate the authentication request.
impl ValidateRequest for AuthenticationPostRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !IDENTIFIER_REGEX.is_match(&self.identifier) {
            return Err(Box::new(ApiError {
                message: "Identifier field is invalid.".to_string(),
            }));
        }

        if !PASSWORD_REGEX.is_match(&self.password) {
            return Err(Box::new(ApiError {
                message: "Password field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to the authentication request (POST) when using session_version 3.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationPostResponseV3 {
    pub account_id: String,
    pub client_id: String,
    pub lightstreamer_endpoint: String,
    pub oauth_token: OauthToken,
    pub timezone_offset: f64,
}

/// Validate the authentication response.
impl ValidateResponse for AuthenticationPostResponseV3 {}

/// User's session details request with optionally tokens by
/// sending a GET request to the /session endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetailsGetRequest {
    /// Indicates whether to fetch session token headers.
    pub fetch_session_tokens: bool,
}

impl ValidateRequest for SessionDetailsGetRequest {}

/// OAuth access token, which is part of the response to the authentication request
/// when using session_version 3.
#[derive(Debug, Deserialize, Serialize)]
pub struct OauthToken {
    /// Access token.
    pub access_token: String,
    /// Access token expiry in seconds.
    pub expires_in: String,
    /// Refresh token.
    pub refresh_token: String,
    /// Scope of the access token.
    pub scope: String,
    /// Token type.
    pub token_type: String,
}

/// User's session details response. Response to the GET /session request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetailsGetResponse {
    /// Active account identifier.
    pub account_id: String,
    /// Client identifier.
    pub client_id: String,
    /// Currency.
    pub currency: String,
    /// Lightstreamer endpoint.
    pub lightstreamer_endpoint: String,
    /// Locale.
    pub locale: String,
    /// Timezone offset relative to UTC (in hours).
    pub timezone_offset: f64,
}

/// Validate the session details response.
impl ValidateResponse for SessionDetailsGetResponse {}

/// The encryption key to use in order to send the user password in an encrypted form.
/// Response to the GET /session/encryption-key request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEncryptionKeyGetResponse {
    /// Encryption key in Base 64 format.
    pub encryption_key: String,
    /// Current timestamp in milliseconds since epoch.
    pub time_stamp: u64,
}

/// Validate the session encryption key response.
impl ValidateResponse for SessionEncryptionKeyGetResponse {}

/// Request a new session token by sending a POST request
/// to the /session/refresh-token endpoint.
#[derive(Debug, Serialize)]
pub struct SessionRefreshTokenPostRequest {
    /// Refresh token
    pub refresh_token: String,
}

/// Validate the session refresh token request.
impl ValidateRequest for SessionRefreshTokenPostRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.refresh_token.is_empty() {
            return Err(Box::new(ApiError {
                message: "Refresh token field is empty.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Access token response. Response to the POST /session/refresh-token request.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionRefreshTokenPostResponse {
    /// Access token.
    pub access_token: String,
    /// Access token expiry in seconds.
    pub expires_in: String,
    /// Refresh token.
    pub refresh_token: String,
    /// Scope of the access token.
    pub scope: String,
    /// Token type.
    pub token_type: String,
}

/// Validate the session encryption key response.
impl ValidateResponse for SessionRefreshTokenPostResponse {}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// WORKINGORDERS ENDPOINT MODELS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Specific working order.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrder {
    market_data: MarketData,
    working_order_data: WorkingOrderData,
}

/// Working order data.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderData {
    /// Local date and time when the order was created. Format is yyyy/MM/dd kk:mm:ss:SSS.
    pub created_date: String,
    /// Date and time when the order was created.
    #[serde(rename = "createdDateUTC")]
    pub created_date_utc: String,
    /// Currency ISO code.
    pub currency_code: String,
    /// Deal identifier.
    pub deal_id: String,
    /// Deal direction.
    pub direction: Direction,
    /// True if this is a DMA (Direct Market Access) working order.
    pub dma: Option<bool>,
    /// Instrument epic identifier.
    pub epic: Option<String>,
    /// The date and time the working order will be deleted if not triggered till then. Date format is yyyy/MM/dd hh:mm.
    pub good_till_date: Option<String>,
    #[serde(rename = "goodTillDateISO")]
    /// The date and time the working order will be deleted if not triggered till then.
    pub good_till_date_iso: Option<String>,
    /// True if controlled risk.
    pub guaranteed_stop: bool,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Limited risk premium.
    pub limited_risk_premium: Option<f64>,
    /// Price at which to execute the trade.
    pub order_level: Option<f64>,
    /// Order size.
    pub order_size: Option<f64>,
    /// Working order type.
    pub order_type: WorkingOrderType,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Describes the type of time in force for a given order
    pub time_in_force: WorkingOrderTimeInForce,
}

/// Request to delete a working order by sending a DELETE request to the /workingorders/otc/{dealId} endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderDeleteRequest {
    /// Deal identifier.
    pub deal_id: String,
}

impl ValidateRequest for WorkingOrderDeleteRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !DEAL_ID_REGEX.is_match(&self.deal_id) {
            return Err(Box::new(ApiError {
                message: "Deal ID field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to working order deletion request through the DELETE /workingorders/otc/{dealId} endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderDeleteResponse {
    /// Deal reference of the transaction.
    pub deal_reference: String,
}

impl ValidateResponse for WorkingOrderDeleteResponse {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !DEAL_REFERENCE_REGEX.is_match(&self.deal_reference) {
            return Err(Box::new(ApiError {
                message: "Deal reference field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Request to create a new working order by sending a POST request to the /workingorders/otc endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderPostRequest {
    /// Currency. Restricted to available instrument currencies.
    pub currency_code: String,
    /// A user-defined reference identifying the submission of the order.
    pub deal_reference: Option<String>,
    /// Deal direction.
    pub direction: Direction,
    /// Instrument epic.
    pub epic: String,
    /// Expiry.
    pub expiry: String,
    /// Force open.
    pub force_open: Option<bool>,
    /// Good till date - This accepts two possible formats either yyyy/mm/dd hh:mm:ss in UTC Time
    /// or Unix Timestamp in milliseconds.
    pub good_till_date: Option<String>,
    /// Guaranteed stop.
    pub guaranteed_stop: bool,
    /// Deal level.
    pub level: f64,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Order size.
    pub size: f64,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// Time in force.
    pub time_in_force: WorkingOrderTimeInForce,
    /// Working order type.
    pub r#type: WorkingOrderType,
}

impl ValidateRequest for WorkingOrderPostRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: field currency_code follows pattern(regexp="[A-Z]{3}").
        if !CURRENCY_CODE_REGEX.is_match(&self.currency_code) {
            return Err(Box::new(ApiError {
                message: "Currency code field is invalid.".to_string(),
            }));
        }

        // Constraint: field deal_reference follows pattern(regexp="[A-Za-z0-9_\\-]{1,30}")].
        if let Some(deal_reference) = &self.deal_reference {
            if !DEAL_REFERENCE_REGEX.is_match(deal_reference) {
                return Err(Box::new(ApiError {
                    message: "Deal reference field is invalid.".to_string(),
                }));
            }
        }

        // Constraint: field epic follows pattern(regexp="[A-Za-z0-9._]{6,30}").
        if !EPIC_REGEX.is_match(&self.epic) {
            return Err(Box::new(ApiError {
                message: "Epic field is invalid.".to_string(),
            }));
        }

        // Constraint: field expiry follows pattern(regexp="(\\d{2}-)?[A-Z]{3}-\\d{2}|-|DFB").
        if !EXPIRY_REGEX.is_match(&self.expiry) {
            return Err(Box::new(ApiError {
                message: "Expiry field is invalid.".to_string(),
            }));
        }

        // Constraint: check precision of size is not more than 12 decimal places.
        let size_str = format!("{}", self.size);
        let parts: Vec<&str> = size_str.split('.').collect();
        if parts.len() == 2 && parts[1].len() > 12 {
            return Err(Box::new(ApiError {
                message: "Size field has more thatn 12 decimal places.".to_string(),
            }));
        }

        // Constraint: if guaranteed_stop equals true, then set stop_distance.
        if self.guaranteed_stop == true && self.stop_distance.is_none() {
            return Err(Box::new(ApiError {
                message: "stop_distance field is required when guaranteed_stop is true."
                    .to_string(),
            }));
        }

        // Constraint: If time_in_force equals GOOD_TILL_DATE, then set good_till_date field.
        match &self.time_in_force {
            WorkingOrderTimeInForce::GoodTillDate => {
                if self.good_till_date.is_none() {
                    return Err(Box::new(ApiError {
                        message:
                            "good_till_date field is required when time_in_force is GOOD_TILL_DATE."
                                .to_string(),
                    }));
                }
            }
            _ => {}
        }

        // Constraint: set only one of {limit_level, limit_distance}.
        if self.limit_level.is_some() && self.limit_distance.is_some() {
            return Err(Box::new(ApiError {
                message: "Set only one of {limit_level, limit_distance}.".to_string(),
            }));
        }

        // Constraint: set only one of {stop_level,stop_distance}.
        if self.stop_level.is_some() && self.stop_distance.is_some() {
            return Err(Box::new(ApiError {
                message: "Set only one of {stop_level, stop_distance}.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to working order creation request through the POST /workingorders/otc endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderPostResponse {
    /// Deal reference of the transaction.
    pub deal_reference: String,
}

impl ValidateResponse for WorkingOrderPostResponse {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !DEAL_REFERENCE_REGEX.is_match(&self.deal_reference) {
            return Err(Box::new(ApiError {
                message: "Deal reference field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Request to update a working order by sending a PUT request to the /workingorders/otc/{dealId} endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderPutRequest {
    /// Good till date - This accepts two possible formats either yyyy/mm/dd hh:mm:ss
    /// in UTC Time or Unix Timestamp in milliseconds.
    pub good_till_date: Option<String>,
    /// True if a guaranteed stop is required.
    pub guaranteed_stop: Option<bool>,
    /// Deal level.
    pub level: f64,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// Time in force.
    pub time_in_force: WorkingOrderTimeInForce,
    /// Working order type.
    pub r#type: WorkingOrderType,
}

impl ValidateRequest for WorkingOrderPutRequest {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Constraint: if guaranteed_stop equals true, then set stop_level.
        if self.guaranteed_stop == Some(true) && self.stop_level.is_none() {
            return Err(Box::new(ApiError {
                message: "stop_level must be set when guaranteed_stop is true.".to_string(),
            }));
        }

        // Constraint: if time_in_force equals GOOD_TILL_DATE, then set good_till_date field.
        match &self.time_in_force {
            WorkingOrderTimeInForce::GoodTillDate => {
                if self.good_till_date.is_none() {
                    return Err(Box::new(ApiError {
                        message:
                            "good_till_date field is required when time_in_force is GOOD_TILL_DATE."
                                .to_string(),
                    }));
                }
            }
            _ => {}
        }

        // Constraint: set only one of {limit_level, limit_distance}.
        if self.limit_level.is_some() && self.limit_distance.is_some() {
            return Err(Box::new(ApiError {
                message: "Set only one of {limit_level, limit_distance}.".to_string(),
            }));
        }

        // Constraint: set only one of {stop_level, stop_distance}.
        if self.stop_level.is_some() && self.stop_distance.is_some() {
            return Err(Box::new(ApiError {
                message: "Set only one of {stop_level, stop_distance}.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to working order update request through the PUT /workingorders/otc/{dealId} endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderPutResponse {
    /// Deal reference of the transaction.
    pub deal_reference: String,
}

impl ValidateResponse for WorkingOrderPutResponse {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !DEAL_REFERENCE_REGEX.is_match(&self.deal_reference) {
            return Err(Box::new(ApiError {
                message: "Deal reference field is invalid.".to_string(),
            }));
        }

        Ok(())
    }
}

/// Response to the GET /workingorders request, which returns a list of working orders for the active account.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrdersGetResponse {
    /// List of working orders.
    pub working_orders: Vec<WorkingOrder>,
}

impl ValidateResponse for WorkingOrdersGetResponse {}

/// Working order type.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingOrderType {
    /// Limit working order.
    Limit,
    #[default]
    /// Stop working order.
    Stop,
}

/// Describes the type of time in force for a given order.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingOrderTimeInForce {
    #[default]
    /// Good until cancelled.
    GoodTillCancelled,
    /// Good until specified date.
    GoodTillDate,
}
