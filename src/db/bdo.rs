use bdo_enhancement_calc::requests::{
    requests::{
        CategorySearchItem, ItemInfo, ItemSearchInfo, SearchResultItem, TrendingItem, WaitListItem,
    },
    Region,
};

use super::get_database;

pub fn get_timestamp_secs() -> i64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH)
        .expect("time has gone backwards A LOT")
        .as_secs() as i64
}

pub async fn insert_search_result(item: SearchResultItem, region: Region) -> Result<(), ()> {
    let database = get_database();

    let region = region as i64;
    let item_id = item.item_id as i64;
    let stock = item.stock as i64;
    let base_price = item.base_price as i64;
    let name = item.name;
    let grade = item.grade as i64;
    let godr_ayed = if item.godr_ayed { 1 } else { 0 };
    let last_update_time = get_timestamp_secs();
    let ins = sqlx::query!(
        "INSERT OR REPLACE INTO SearchResultItems
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        region,
        item_id,
        stock,
        base_price,
        name,
        grade,
        godr_ayed,
        last_update_time
    )
    .execute(database)
    .await;

    match ins {
        Ok(_) => return Ok(()),
        Err(e) => {
            tracing::warn!("Failed to insert data into the database: {}", e);
            return Err(());
        }
    }
}

pub async fn get_search_result(
    item_id: u64,
    region: Region,
) -> Result<(SearchResultItem, u64), ()> {
    let database = get_database();
    let region = region as i64;
    let item_id = item_id as i64;
    let data = sqlx::query!(
        "
        SELECT *
        FROM SearchResultItems
        WHERE item_id = ?
        AND region = ?",
        item_id,
        region,
    )
    .fetch_one(database)
    .await;

    let Ok(d) = data else {
        tracing::warn!(
            "Failed to retrieve item with id: {} from the database",
            item_id
        );
        return Err(());
    };

    let item = SearchResultItem::new(
        d.item_id as u64,
        d.stock as u64,
        d.base_price as u64,
        d.name,
        d.grade as u64,
        d.godr_ayed != 0,
    );

    Ok((item, d.last_update_time as u64))
}

pub async fn insert_item_search_info(item: ItemSearchInfo, region: Region) -> Result<(), ()> {
    let database = get_database();

    let region = region as i64;
    let item_id = item.item_id as i64;
    let base_price = item.base_price as i64;
    let total_trade_count = item.total_trade_count as i64;
    let key_type = item.key_type as i64;
    let sub_key = item.sub_key as i64;
    let count = item.count as i64;
    let name = item.name;
    let grade = item.grade as i64;
    let main_category = item.main_category as i64;
    let sub_category = item.sub_category as i64;
    let enhancement_level = item.enhancement_level as i64;

    let godr_ayed = if item.godr_ayed { 1 } else { 0 };
    let last_update_time = get_timestamp_secs();
    let ins = sqlx::query!(
        "INSERT OR REPLACE INTO ItemSearchInfo
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        region,
        item_id,
        base_price,
        total_trade_count,
        key_type,
        sub_key,
        count,
        name,
        grade,
        main_category,
        sub_category,
        enhancement_level,
        godr_ayed,
        last_update_time
    )
    .execute(database)
    .await;

    match ins {
        Ok(_) => return Ok(()),
        Err(e) => {
            tracing::warn!("Failed to insert data into the database: {}", e);
            return Err(());
        }
    }
}

pub async fn get_item_search_info(
    item_id: u64,
    region: Region,
) -> Result<(ItemSearchInfo, u64), ()> {
    let database = get_database();
    let region = region as i64;
    let item_id = item_id as i64;
    let data = sqlx::query!(
        "
        SELECT *
        FROM ItemSearchInfo
        WHERE item_id = ?
        AND region = ?",
        item_id,
        region
    )
    .fetch_one(database)
    .await;

    let Ok(d) = data else {
        tracing::warn!(
            "Failed to retrieve item with id: {} from the database",
            item_id
        );
        return Err(());
    };

    let item = ItemSearchInfo::new(
        d.item_id as u64,
        d.base_price as u64,
        d.total_trade_count as u64,
        d.key_type as u64,
        d.sub_key as u64,
        d.count as u64,
        d.name,
        d.grade as u64,
        d.main_category as u64,
        d.sub_category as u64,
        d.enhancement_level as u64,
        d.godr_ayed != 0,
    );

    Ok((item, d.last_update_time as u64))
}

pub async fn insert_category_search_item(
    item: CategorySearchItem,
    region: Region,
) -> Result<(), ()> {
    let database = get_database();

    let region = region as i64;
    let item_id = item.item_id as i64;
    let stock = item.stock as i64;
    let name = item.name;
    let grade = item.grade as i64;
    let godr_ayed = if item.godr_ayed { 1 } else { 0 };
    let base_price = item.base_price as i64;
    let last_update_time = get_timestamp_secs();

    let ins = sqlx::query!(
        "INSERT INTO CategorySearchItems
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        region,
        item_id,
        stock,
        name,
        grade,
        godr_ayed,
        base_price,
        last_update_time,
    )
    .execute(database)
    .await;

    match ins {
        Ok(_) => return Ok(()),
        Err(e) => {
            tracing::warn!("Failed to insert data into the database: {}", e);
            return Err(());
        }
    }
}

pub async fn get_category_search_item(
    item_id: u64,
    region: Region,
) -> Result<(CategorySearchItem, u64), ()> {
    let database = get_database();
    let region = region as i64;
    let item_id = item_id as i64;
    let data = sqlx::query!(
        "
        SELECT *
        FROM CategorySearchItems
        WHERE item_id = ?
        AND region = ?",
        item_id,
        region
    )
    .fetch_one(database)
    .await;

    let Ok(d) = data else {
        tracing::warn!(
            "Failed to retrieve item with id: {} from the database",
            item_id
        );
        return Err(());
    };

    let item = CategorySearchItem::new(
        d.item_id as u64,
        d.stock as u64,
        d.name,
        d.grade as u64,
        d.godr_ayed != 0,
        d.base_price as u64,
    );

    Ok((item, d.last_update_time as u64))
}

pub async fn insert_trending_item(item: TrendingItem, region: Region) -> Result<(), ()> {
    let database = get_database();

    let region = region as i64;
    let item_id = item.item_id as i64;
    let subtype = item.subtype as i64;
    let base_price = item.base_price as i64;
    let total_trade_count = item.total_trade_count as i64;
    let fluctuation_type = item.fluctuation_type as i64;
    let fluctuation_price = item.fluctuation_price as i64;
    let key_type = item.key_type as i64;
    let sub_key = item.sub_key as i64;
    let count = item.count as i64;
    let name = item.name;
    let grade = item.grade as i64;
    let main_category = item.main_category as i64;
    let sub_category = item.sub_category as i64;
    let enhancement_level = item.enhancement_level as i64;
    let godr_ayed = if item.godr_ayed { 1 } else { 0 };
    let last_update_time = get_timestamp_secs();

    let ins = sqlx::query!(
        "INSERT OR REPLACE INTO TrendingItem
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        region,
        item_id,
        subtype,
        base_price,
        total_trade_count,
        fluctuation_type,
        fluctuation_price,
        key_type,
        sub_key,
        count,
        name,
        grade,
        main_category,
        sub_category,
        enhancement_level,
        godr_ayed,
        last_update_time,
    )
    .execute(database)
    .await;

    match ins {
        Ok(_) => return Ok(()),
        Err(e) => {
            tracing::warn!("Failed to insert data into the database: {}", e);
            return Err(());
        }
    }
}

pub async fn get_trending_item(item_id: u64, region: Region) -> Result<(TrendingItem, u64), ()> {
    let database = get_database();
    let region = region as i64;
    let item_id = item_id as i64;
    let data = sqlx::query!(
        "
        SELECT *
        FROM TrendingItem
        WHERE item_id = ?
        AND region = ?",
        item_id,
        region,
    )
    .fetch_one(database)
    .await;

    let Ok(d) = data else {
        tracing::warn!(
            "Failed to retrieve item with id: {} from the database",
            item_id
        );
        return Err(());
    };

    let item = TrendingItem::new(
        d.subtype as u64,
        d.base_price as u64,
        d.total_trade_count as u64,
        d.fluctuation_type as u64,
        d.fluctuation_price as u64,
        d.key_type as u64,
        d.item_id as u64,
        d.sub_key as u64,
        d.count as u64,
        d.name,
        d.grade as u64,
        d.main_category as u64,
        d.sub_category as u64,
        d.enhancement_level as u64,
        d.godr_ayed != 0,
    );

    Ok((item, d.last_update_time as u64))
}

pub async fn insert_item_info(item: ItemInfo, item_id: u64, region: Region) -> Result<(), ()> {
    let database = get_database();

    let region = region as i64;
    let item_id = item_id as i64;
    let base_price = item.base_price as i64;
    let enchant_group = item.enchant_group as i64;
    let enchant_max_group = item.enchant_max_group as i64;
    let enchant_material_id = item.enchant_material_id as i64;
    let enchant_material_base_price = item.enchant_material_base_price as i64;
    let enchant_need_count = item.enchant_need_count as i64;
    let max_user_registrations = item.max_user_registrations as i64;
    let sell_count_market = item.sell_count_market as i64;
    let buy_ref_count_market = item.buy_ref_count_market as i64;
    let buy_count_market = item.buy_count_market as i64;
    let bidding_sell_count = item.bidding_sell_count as i64;
    let count_value = item.count_value as i64;
    let sell_max_count = item.sell_max_count as i64;
    let buy_max_count = item.buy_max_count as i64;
    let is_wait_item = item.is_wait_item as i64;
    let price_history = serde_json::to_string(&item.price_history).unwrap();
    let price_list = serde_json::to_string(&item.price_list).unwrap();
    let market_listings = serde_json::to_string(&item.market_listings).unwrap();
    let last_update_time = get_timestamp_secs();

    let ins = sqlx::query!(
        "INSERT OR REPLACE INTO ItemInfo
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        region,
        item_id,
        base_price,
        enchant_group,
        enchant_max_group,
        enchant_material_id,
        enchant_material_base_price,
        enchant_need_count,
        max_user_registrations,
        sell_count_market,
        buy_ref_count_market,
        buy_count_market,
        bidding_sell_count,
        count_value,
        sell_max_count,
        buy_max_count,
        is_wait_item,
        price_history,
        price_list,
        market_listings,
        last_update_time,
    )
    .execute(database)
    .await;

    match ins {
        Ok(_) => return Ok(()),
        Err(e) => {
            tracing::warn!("Failed to insert data into the database: {}", e);
            return Err(());
        }
    }
}

pub async fn get_item_info(item_id: u64, region: Region) -> Result<(ItemInfo, u64), ()> {
    let database = get_database();
    let region = region as i64;
    let item_id = item_id as i64;
    let data = sqlx::query!(
        "
        SELECT *
        FROM ItemInfo
        WHERE item_id = ?
        AND region = ?",
        item_id,
        region
    )
    .fetch_one(database)
    .await;

    let Ok(d) = data else {
        tracing::warn!(
            "Failed to retrieve item with id: {} from the database",
            item_id
        );
        return Err(());
    };

    let item = ItemInfo::new(
        serde_json::from_str(&d.price_list).unwrap(),
        serde_json::from_str(&d.market_listings).unwrap(),
        d.base_price as u64,
        d.enchant_group as u64,
        d.enchant_max_group as u64,
        d.enchant_material_id as u64,
        d.enchant_material_base_price as u64,
        d.enchant_need_count as u64,
        d.max_user_registrations as u64,
        d.sell_count_market as u64,
        d.buy_ref_count_market as u64,
        d.buy_count_market as u64,
        d.bidding_sell_count as u64,
        d.count_value as u64,
        d.sell_max_count as u64,
        d.buy_max_count as u64,
        d.is_wait_item != 0,
        0,
        serde_json::from_str(&d.market_listings).unwrap(),
    );

    Ok((item, d.last_update_time as u64))
}
