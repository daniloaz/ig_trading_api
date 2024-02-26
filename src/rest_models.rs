use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
	pub account_alias: Option<String>,
	pub account_id: String,
	pub account_name: String,
	pub account_type: AccountType,
	pub balance: Balance,
	pub can_transfer_from: bool,
	pub can_transfer_to: bool,
	pub currency: String,
	pub preferred: bool,
	pub status: AccountStatus
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
	pub accounts: Vec<Account>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
	Disabled,
	Enabled,
	SuspendedFromDealing
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSwitchRequest {
	pub account_id: String,
	pub default_account: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSwitchResponse {
	pub dealing_enabled: bool,
	pub has_active_demo_accounts: bool,
	pub has_active_live_accounts: bool,
	pub trailing_stops_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
	Cfd,
	Physical,
	Spreadbet
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationRequest {
	pub identifier: String,
	pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponseV3 {
	pub account_id: String,
	pub client_id: String,
	pub lightstreamer_endpoint: String,
	pub oauth_token: OauthToken,
	pub timezone_offset: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
	pub available: f64,
	pub balance: f64,
	pub deposit: f64,
	pub profit_loss: f64
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
	pub trailing_stops_enabled: bool
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoryQuery {
	pub from: Option<NaiveDateTime>,
	pub to: Option<NaiveDateTime>,
	pub detailed: Option<bool>,
	pub deal_id: Option<String>,
	pub filter: Option<String>,
	pub page_size: Option<u32>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistory {
	pub activities: Vec<Activity>,
	pub metadata: ActivityMetadata
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
	pub channel: Channel,
	pub date: String,
	pub deal_id: String,
	pub description: String,
	pub details: Option<ActivityDetails>,
	pub epic: String,
	pub period: String,
	pub status: ActivityStatus,
	pub r#type: ActivityType
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Channel {
	Dealer,
	Mobile,
	PublicFixApi,
	PublicWebApi,
	System,
	Web
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDetails {
	pub actions: Vec<ActivityAction>,
	pub currency: String,
	pub deal_reference: String,
	pub direction: Direction,
	pub good_till_date: String,
	pub guaranteed_stop: bool,
	pub level: f64,
	pub limit_distance: f64,
	pub limit_level: f64,
	pub market_name: String,
	pub size: f64,
	pub stop_distance: f64,
	pub stop_level: f64,
	pub trailing_step: f64,
	pub trailing_stop_distance: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAction {
	pub action_type: ActivityActionType,
	pub affected_deal_id: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityActionType {
	LimitOrderAmended,
	LimitOrderDeleted,
	LimitOrderFilled,
	LimitOrderOpened,
	LimitOrderRolled,
	PositionClosed,
	PositionDeleted,
	PositionOpened,
	PositionPartiallyClosed,
	PositionRolled,
	StopLimitAmended,
	StopOrderAmended,
	StopOrderDeleted,
	StopOrderFilled,
	StopOrderOpened,
	StopOrderRolled,
	Unknown,
	WorkingOrderDeleted
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
	Buy,
	Sell
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityStatus {
	Accepted,
	Rejected,
	Unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
	EditStopAndLimit,
	Position,
	System,
	WorkingOrder
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMetadata {
	pub paging: ActivityPaging
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPaging {
	pub next: Option<String>,
	pub size: u32
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryQuery {
	pub r#type: Option<TransactionType>,
	pub from: Option<NaiveDateTime>,
	pub to: Option<NaiveDateTime>,
	pub max_span_seconds: Option<u64>,
	pub page_size: Option<u32>,
	pub page_number: Option<u32>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
	All,
	AllDeal,
	Deposit,
	Withdrawal
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistory {
	pub metadata: TransactionMetadata,
	pub transactions: Vec<Transaction>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetadata {
	pub page_data: TransactionPaging,
	pub size: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPaging {
	pub page_number: u32,
	pub page_size: u32,
	pub total_pages: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	pub cash_transaction: bool,
	pub close_level: String,
	pub currency: String,
	pub date: String,
	pub date_utc: String,
	pub instrument_name: String,
	pub open_date_utc: String,
	pub open_level: String,
	pub period: String,
	pub profit_and_loss: String,
	pub reference: String,
	pub size: String,
	pub transaction_type: String
}

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
pub struct DealConfirmation {
	pub affected_deals: Vec<AffectedDeal>,
	pub date: String,
	pub deal_id: String,
	pub deal_reference: String,
	pub deal_status: DealStatus,
	pub direction: Direction,
	pub epic: String,
	pub expiry: String,
	pub guaranteed_stop: bool,
	pub level: f64,
	pub limit_distance: f64,
	pub limit_level: f64,
	pub profit: f64,
	pub profit_currency: String,
	pub reason: DealReason,
	pub size: f64,
	pub status: PositionStatus,
	pub stop_distance: f64,
	pub stop_level: f64,
	pub trailing_stop: bool
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedDeal {
	pub deal_id: String,
	pub status: AffectedDealStatus
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AffectedDealStatus {
	Amended,
	Deleted,
	FullyClosed,
	Opened,
	PartiallyClosed
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DealStatus {
	Accepted,
	Rejected
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DealReason {
	AccountNotEnabledToTrading,
	AttachedOrderLevelError,
	AttachedOrderTrailingStopError,
	CannotChangeStopType,
	CannotRemoveStop,
	ClosingOnlyTradesAcceptedOnThisMarket,
	ClosingsOnlyAccount,
	ConflictingOrder,
	ContactSupportInstrumentError,
	CrSpacing,
	DuplicateOrderError,
	ExchangeManualOverride,
	ExpiryLessThanSprintMarketMinExpiry,
	FinanceRepeatDealing,
	ForceOpenOnSameMarketDifferentCurrency,
	GeneralError,
	GoodTillDateInThePast,
	InstrumentNotFound,
	InstrumentNotTradeableInThisCurrency,
	InsufficientFunds,
	LevelToleranceError,
	LimitOrderWrongSideOfMarket,
	ManualOrderTimeout,
	MarginError,
	MarketClosed,
	MarketClosedWithEdits,
	MarketClosing,
	MarketNotBorrowable,
	MarketOffline,
	MarketOrdersNotAllowedOnInstrument,
	MarketPhoneOnly,
	MarketRolled,
	MarketUnavailableToClient,
	MaxAutoSizeExceeded,
	MinimumOrderSizeError,
	MoveAwayOnlyLimit,
	MoveAwayOnlyStop,
	MoveAwayOnlyTriggerLevel,
	NcrPositionsOnCrAccount,
	OpposingDirectionOrdersNotAllowed,
	OpposingPositionsNotAllowed,
	OrderDeclined,
	OrderLocked,
	OrderNotFound,
	OrderSizeCannotBeFilled,
	OverNormalMarketSize,
	PartialyClosedPositionNotDeleted,
	PositionAlreadyExistsInOppositeDirection,
	PositionNotAvailableToCancel,
	PositionNotAvailableToClose,
	PositionNotFound,
	RejectCfdOrderOnSpreadbetAccount,
	RejectSpreadbetOrderOnCfdAccount,
	SizeIncrement,
	SprintMarketExpiryAfterMarketClose,
	StopOrLimitNotAllowed,
	StopRequiredError,
	StrikeLevelTolerance,
	Success,
	TrailingStopNotAllowed,
	Unknown,
	WrongSideOfMarket
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionStatus {
	Amended,
	Closed,
	Deleted,
	Open,
	PartiallyClosed
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Positions {
	pub positions: Vec<Position>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
	pub market: MarketData,
	pub position: PositionData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
	pub bid: Option<f64>,
	pub delay_time: f64,
	pub epic: String,
	pub exchange_id: Option<String>,
	pub expiry: String,
	pub high: Option<f64>,
	pub instrument_name: String,
	pub instrument_type: InstrumentType,
	pub lot_size: Option<f64>,
	pub low: Option<f64>,
	pub market_status: MarketStatus,
	pub net_change: f64,
	pub offer: Option<f64>,
	pub percentage_change: f64,
	pub scaling_factor: f64,
	pub streaming_prices_available: bool,
	pub update_time: String,
	#[serde(rename = "updateTimeUTC")]
	pub update_time_utc: String
}

#[derive(Debug, Deserialize)]
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
	Unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketStatus {
	Closed,
	EditsOnly,
	Offline,
	OnAuction,
	OnAuctionNoEdits,
	Suspended,
	Tradeable
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
	pub contract_size: f64,
	pub controlled_risk: bool,
	pub created_date: String,
	pub created_date_utc: String,
	pub currency: String,
	pub deal_id: String,
	pub deal_reference: String,
	pub direction: Direction,
	pub level: f64,
	pub limit_level: f64,
	pub limited_risk_premium: f64,
	pub size: f64,
	pub stop_level: f64,
	pub trailing_step: f64,
	pub trailing_stop_distance: f64
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePosition {
	pub deal_id: Option<String>,
	pub direction: Option<Direction>,
	pub epic: Option<String>,
	pub expiry: Option<String>,
	pub level: Option<f64>,
	pub order_type: Option<OrderType>,
	pub quote_id: Option<String>,
	pub size: Option<f64>,
	pub time_in_force: Option<TimeInForce>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
	Limit,
	Market,
	Quote
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
	ExecuteAndEliminate,
	FillOrKill
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealRef {
	pub deal_reference: String
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePosition {
	pub currency_code: Option<String>,
	pub deal_reference: Option<String>,
	pub direction: Option<Direction>,
	pub epic: Option<String>,
	pub expiry: Option<String>,
	pub force_open: Option<bool>,
	pub guaranteed_stop: Option<bool>,
	pub level: Option<f64>,
	pub limit_distance: Option<f64>,
	pub limit_level: Option<f64>,
	pub order_type: Option<OrderType>,
	pub quote_id: Option<String>,
	pub size: Option<f64>,
	pub stop_distance: Option<f64>,
	pub stop_level: Option<f64>,
	pub time_in_force: Option<TimeInForce>,
	pub trailing_stop: Option<bool>,
	pub trailing_stop_increment: Option<f64>
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePosition {
	pub guaranteed_stop: Option<bool>,
	pub limit_level: Option<f64>,
	pub stop_level: Option<f64>,
	pub trailing_stop: Option<bool>,
	pub trailing_stop_distance: Option<f64>,
	pub trailing_stop_increment: Option<f64>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintMarketPositions {
	pub sprint_market_positions: Vec<SprintMarketPosition>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintMarketPosition {
	pub created_date: String,
	pub currency: String,
	pub deal_id: String,
	pub description: String,
	pub direction: Direction,
	pub epic: String,
	pub expiry_time: String,
	pub instrument_name: String,
	pub market_status: MarketStatus,
	pub payout_amount: f64,
	pub size: f64,
	pub strike_level: f64
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSprintMarketPosition {
	pub deal_reference: Option<String>,
	pub direction: Option<Direction>,
	pub epic: Option<String>,
	pub expiry_period: Option<SprintMarketExpiryPeriod>,
	pub size: Option<f64>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SprintMarketExpiryPeriod {
	FiveMinutes,
	OneMinute,
	SixtyMinutes,
	TwentyMinutes,
	TwoMinutes
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrders {
	pub working_orders: Vec<WorkingOrder>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrder {
	market_data: MarketData,
	working_order_data: WorkingOrderData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingOrderData {
	pub created_date: String,
	#[serde(rename = "createdDateUTC")]
	pub created_date_utc: String,
	pub currency_code: String,
	pub deal_id: String,
	pub direction: Direction,
	pub dma: bool,
	pub epic: String,
	pub good_till_date: String,
	#[serde(rename = "goodTillDateISO")]
	pub good_till_date_iso: String,
	pub guaranteed_stop: bool,
	pub limit_distance: f64,
	pub limited_risk_premium: f64,
	pub order_level: f64,
	pub order_size: f64,
	pub order_type: WorkingOrderType,
	pub stop_distance: f64,
	pub time_in_force: WorkingOrderTimeInForce
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingOrderType {
	Limit,
	Stop
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingOrderTimeInForce {
	GoodTillCancelled,
	GoodTillDate
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkingOrder {
	pub currency_code: Option<String>,
	pub deal_reference: Option<String>,
	pub direction: Option<Direction>,
	pub epic: Option<String>,
	pub expiry: Option<String>,
	pub force_open: Option<bool>,
	pub good_till_date: Option<String>,
	pub guaranteed_stop: Option<bool>,
	pub level: Option<f64>,
	pub limit_distance: Option<f64>,
	pub limit_level: Option<f64>,
	pub size: Option<f64>,
	pub stop_distance: Option<f64>,
	pub stop_level: Option<f64>,
	pub time_in_force: Option<WorkingOrderTimeInForce>,
	pub r#type: Option<WorkingOrderType>
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkingOrder {
	pub good_till_date: Option<String>,
	pub guaranteed_stop: Option<bool>,
	pub level: Option<f64>,
	pub limit_distance: Option<f64>,
	pub limit_level: Option<f64>,
	pub stop_distance: Option<f64>,
	pub stop_level: Option<f64>,
	pub time_in_force: Option<WorkingOrderTimeInForce>,
	pub r#type: Option<WorkingOrderType>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkResponse {
	pub status: ResponseStatus
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseStatus {
	Success
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
	pub status: ApplicationStatus
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationStatus {
	Disabled,
	Enabled,
	Revoked
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplication {
	pub allowance_account_overall: f64,
	pub allowance_account_trading: f64,
	pub api_key: String,
	pub status: ApplicationStatus
}

/// User's session details request with optionally tokens.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetailsRequest {
	/// Indicates whether to fetch session token headers.
	pub fetch_session_tokens: bool,
}

/// User's session details response.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetailsResponse {
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
	pub timezone_offset: f64
}

/// The encryption key to use in order to send the user password in an encrypted form
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEncryptionKeyResponse {
	/// Encryption key in Base 64 format.
	pub encryption_key: String,
	/// Current timestamp in milliseconds since epoch.
	pub timestamp: u64,
}

/// OAuth access token.
#[derive(Debug, Deserialize)]
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
	pub token_type: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCategory {
    pub markets: Option<Vec<MarketData3>>,
    pub nodes: Option<Vec<MarketNode>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData3 {
    pub bid: f64,
    pub delay_time: f64,
    pub epic: String,
    pub expiry: String,
    pub high: f64,
    pub instrument_name: String,
    pub instrument_type: InstrumentType,
    pub lot_size: f64,
    pub low: f64,
    pub market_status: MarketStatus,
    pub net_change: f64,
    pub offer: f64,
    pub otc_tradeable: bool,
    pub percentage_change: f64,
    pub scaling_factor: f64,
    pub streaming_prices_available: bool,
    pub update_time: String,
    pub update_time_utc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketNode {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct MarketsQuery {
    pub epics: Vec<String>,
    pub filter: Option<MarketDetailsFilterType>,
}

impl Serialize for MarketsQuery {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketDetailsFilterType {
    All,
    SnapshotOnly,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Markets {
    pub market_details: Vec<Market>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub dealing_rules: DealingRules,
    pub instrument: InstrumentDetails,
    pub snapshot: MarketSnapshot,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealingRules {
    pub market_order_preference: MarketOrderPreference,
    pub max_stop_or_limit_distance: DealingRule,
    pub min_controlled_risk_stop_distance: DealingRule,
    pub min_deal_size: DealingRule,
    pub min_normal_stop_or_limit_distance: DealingRule,
    pub min_step_distance: DealingRule,
    pub trailing_stops_preference: TrailingStopsPreference,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketOrderPreference {
    AvailableDefaultOff,
    AvailableDefaultOn,
    NotAvailable,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealingRule {
    pub unit: RuleUnit,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleUnit {
    Percentage,
    Points,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrailingStopsPreference {
    Available,
    NotAvailable,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDetails {
    pub chart_code: String,
    pub contract_size: String,
    pub controlled_risk_allowed: bool,
    pub country: Option<String>,
    pub currencies: Vec<Currency>,
    pub epic: String,
    pub expiry: String,
    pub expiry_details: Option<Expiry>,
    pub force_open_allowed: bool,
    pub limited_risk_premium: DealingRule,
    pub lot_size: f64,
    pub margin_deposit_bands: Vec<DepositBand>,
    pub margin_factor: f64,
    pub margin_factor_unit: RuleUnit,
    pub market_id: String,
    pub name: String,
    pub news_code: String,
    pub one_pip_means: String,
    pub opening_hours: Option<OpeningHours>,
    pub rollover_details: Option<Rollover>,
    pub slippage_factor: SlippageFactor,
    pub special_info: Vec<String>,
    pub sprint_markets_maximum_expiry_time: Option<f64>,
    pub sprint_markets_minimum_expiry_time: Option<f64>,
    pub stops_limits_allowed: bool,
    pub streaming_prices_available: bool,
    pub r#type: InstrumentType,
    pub unit: InstrumentUnit,
    pub value_of_one_pip: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub base_exchange_rate: f64,
    pub code: String,
    pub exchange_rate: f64,
    pub is_default: bool,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expiry {
    pub last_dealing_date: String,
    pub settlement_info: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositBand {
    pub currency: String,
    pub margin: f64,
    pub max: Option<f64>,
    pub min: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    pub market_times: Vec<MarketTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rollover {
    pub last_rollover_time: String,
    pub rollover_info: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTime {
    pub close_time: String,
    pub open_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlippageFactor {
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentUnit {
    Amount,
    Contracts,
    Shares,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSnapshot {
    pub bid: f64,
    pub binary_odds: Option<f64>,
    pub controlled_risk_extra_spread: f64,
    pub decimal_places_factor: f64,
    pub delay_time: f64,
    pub high: f64,
    pub low: f64,
    pub market_status: MarketStatus,
    pub net_change: f64,
    pub offer: f64,
    pub percentage_change: f64,
    pub scaling_factor: f64,
    pub update_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSearch {
    pub markets: Vec<MarketData>
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PricesQuery {
    pub resolution: Option<Resolution>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub max: Option<u32>,
    pub page_size: Option<u32>,
    pub page_number: Option<u32>
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
    Week
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub instrument_type: InstrumentType,
    pub metadata: PriceMetadata,
    pub prices: Vec<Price>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceMetadata {
    pub page_data: PricePaging,
    pub size: f64,
    pub allowance: PriceAllowance
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricePaging {
    pub page_number: u32,
    pub page_size: u32,
    pub total_pages: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAllowance {
    pub allowance_expiry: u32,
    pub remaining_allowance: u32,
    pub total_allowance: u32
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
    pub snapshot_time_utc: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskBid {
    pub ask: f64,
    pub bid: f64,
    pub last_traded: Option<f64>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlists {
	pub watchlists: Vec<Watchlist>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlist {
	pub default_system_watchlist: bool,
	pub deleteable: bool,
	pub editable: bool,
	pub id: String,
	pub name: String
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchlist {
	pub epics: Vec<String>,
	pub name: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchlistResult {
	pub status: CreateWatchlistStatus,
	pub watchlist_id: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateWatchlistStatus {
	Success,
	SuccesNotAllInstrumentsAdded
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToWatchlist {
	pub epic: String
}