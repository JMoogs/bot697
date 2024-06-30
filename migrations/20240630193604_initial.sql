-- These tables mirror the structs from src/requests/requests.rs in bdo-enhancement-calc
-- For boolean fields, any non-zero integer represents true and zero represents false
-- The last_update_time is an additional field used to determine when data needs to be updated from BDO

CREATE TABLE IF NOT EXISTS SearchResultItems (
    region INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    stock INTEGER NOT NULL,
    base_price INTEGER NOT NULL,
    name TEXT NOT NULL,
    grade INTEGER NOT NULL,
    godr_ayed INTEGER NOT NULL,
    last_update_time INTEGER NOT NULL,
    PRIMARY KEY (region, item_id)
);

CREATE TABLE IF NOT EXISTS ItemSearchInfo (
    region INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    base_price INTEGER NOT NULL,
    total_trade_count INTEGER NOT NULL,
    key_type INTEGER NOT NULL,
    sub_key INTEGER NOT NULL,
    count INTEGER NOT NULL,
    name TEXT NOT NULL,
    grade INTEGER NOT NULL,
    main_category INTEGER NOT NULL,
    sub_category INTEGER NOT NULL,
    enhancement_level INTEGER NOT NULL,
    godr_ayed INTEGER NOT NULL,
    last_update_time INTEGER NOT NULL,
    PRIMARY KEY (region, item_id)
);

CREATE TABLE IF NOT EXISTS CategorySearchItems (
    region INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    stock INTEGER NOT NULL,
    name TEXT NOT NULL,
    grade INTEGER NOT NULL,
    godr_ayed INTEGER NOT NULL,
    base_price INTEGER NOT NULL,
    last_update_time INTEGER NOT NULL,
    PRIMARY KEY (region, item_id)
);

CREATE TABLE IF NOT EXISTS TrendingItem (
    region INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    subtype INTEGER NOT NULL,
    base_price INTEGER NOT NULL,
    total_trade_count INTEGER NOT NULL,
    fluctuation_type INTEGER NOT NULL,
    fluctuation_price INTEGER NOT NULL,
    key_type INTEGER NOT NULL,
    sub_key INTEGER NOT NULL,
    count INTEGER NOT NULL,
    name TEXT NOT NULL,
    grade INTEGER NOT NULL,
    main_category INTEGER NOT NULL,
    sub_category INTEGER NOT NULL,
    enhancement_level INTEGER NOT NULL,
    godr_ayed INTEGER NOT NULL,
    last_update_time INTEGER NOT NULL,
    PRIMARY KEY (region, item_id)
);

CREATE TABLE IF NOT EXISTS ItemInfo (
    region INTEGER NOT NULL,
    -- Not given in the struct, must be added
    item_id INTEGER NOT NULL,
    base_price INTEGER NOT NULL,
    enchant_group INTEGER NOT NULL,
    enchant_max_group INTEGER NOT NULL,
    enchant_material_id INTEGER NOT NULL,
    enchant_material_base_price INTEGER NOT NULL,
    enchant_need_count INTEGER NOT NULL,
    max_user_registrations INTEGER NOT NULL,
    sell_count_market INTEGER NOT NULL,
    buy_ref_count_market INTEGER NOT NULL,
    buy_count_market INTEGER NOT NULL,
    bidding_sell_count INTEGER NOT NULL,
    count_value INTEGER NOT NULL,
    sell_max_count INTEGER NOT NULL,
    buy_max_count INTEGER NOT NULL,
    is_wait_item INTEGER NOT NULL,
    -- Serialize this into JSON or similar to store, Vec<PriceHistoryEntry>
    price_history TEXT NOT NULL,
    -- Same here, Vec<u64>
    price_list TEXT NOT NULL,
    -- and here, Vec<MarketListing>
    market_listings TEXT NOT NULL,
    last_update_time INTEGER NOT NULL,
    PRIMARY KEY (region, item_id)
);
