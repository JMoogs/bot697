pub mod items;

use bdo_enhancement_calc::requests::{
    requests::{get_item_details, get_registration_queue, ItemSearchInfo, WaitListItem},
    Context as BDOContext, Region,
};

use crate::db::{
    bdo::{get_item_search_info, get_timestamp_secs, insert_item_search_info},
    get_database,
};
use anyhow::{Context, Result};

const MINIMUM_UPDATE_TIME_SECONDS: i64 = 1800;

fn should_update(last_update_time: i64) -> bool {
    get_timestamp_secs() - last_update_time > MINIMUM_UPDATE_TIME_SECONDS
}

pub async fn get_waitlist(ctx: &BDOContext) -> Result<Vec<WaitListItem>> {
    get_registration_queue(ctx)
        .await
        .context("Failed to get registration queue data")
}

// TODO: Also come back to this - not really sure what the search term should be here
// maybe this function just shouldn't exist at all
// pub async fn get_search_result_or_update(ctx: &Context, item_id: u64) -> Result<SearchResultItem> {
//     let data = get_search_result(item_id).await;

//     if data.is_ok_and(|d| !should_update(d.1)) {
//         return Ok(data.unwrap().0);
//     }

//     search_items(, )
// }

pub async fn get_item_search_info_or_update(
    ctx: &BDOContext,
    item_id: u64,
) -> Result<ItemSearchInfo> {
    let data = get_item_search_info(item_id, ctx.region).await;

    if data.as_ref().is_ok_and(|d| !should_update(d.1 as i64)) {
        return Ok(data.unwrap().0);
    }

    let items = get_item_details(ctx, item_id).await?;
    let return_item = items.iter().find(|i| i.item_id == item_id);
    for item in items.iter() {
        // Silently fail as logging already happens within the function
        let _ = insert_item_search_info(item.clone(), ctx.region).await;
    }

    return_item.cloned().context("Item not found")
}

/// Calculates the tax rate of a user, returning the proportion of money they recieve
pub fn calculate_tax_rate(value_pack: bool, family_fame: u64, merchant_ring: bool) -> f64 {
    let vp = if value_pack { 0.3 } else { 0.0 };
    let merch = if merchant_ring { 0.05 } else { 0.0 };
    let fam_fame = if family_fame >= 7000 {
        0.015
    } else if family_fame >= 4000 {
        0.01
    } else if family_fame >= 1000 {
        0.005
    } else {
        0.0
    };
    0.65 * (1.0 + vp + merch + fam_fame)
}

pub async fn create_user_context(discord_id: u64) -> Result<BDOContext> {
    let discord_id = discord_id as i64;
    let db = get_database();
    let res = sqlx::query!(
        "SELECT region
       FROM Users
       WHERE discord_id = ?",
        discord_id
    )
    .fetch_optional(db)
    .await?;

    match res {
        Some(r) => {
            return BDOContext::init(
                Region::from_i64(r.region).expect("Database should always have valid values"),
            )
            .context("Failed to intialize context")
        }
        None => return None.context("User is not registered"),
    }
}

pub fn format_number(num: u64) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

pub enum GearType {
    Accessory,
    Gear,
}
pub fn get_enhancement_prefix(level: u64, gear_type: GearType) -> String {
    match gear_type {
        GearType::Accessory => match level {
            0 => String::from(""),
            1 => String::from("PRI (I): "),
            2 => String::from("DUO (II): "),
            3 => String::from("TRI (III): "),
            4 => String::from("TET (IV): "),
            5 => String::from("PEN (V): "),
            _ => unreachable!(),
        },
        GearType::Gear => match level {
            0 => String::from(""),
            1 => String::from("+1: "),
            2 => String::from("+2: "),
            3 => String::from("+3: "),
            4 => String::from("+4: "),
            5 => String::from("+5: "),
            6 => String::from("+6: "),
            7 => String::from("+7: "),
            8 => String::from("+8: "),
            9 => String::from("+9: "),
            10 => String::from("+10: "),
            11 => String::from("+11: "),
            12 => String::from("+12: "),
            13 => String::from("+13: "),
            14 => String::from("+14: "),
            15 => String::from("+15: "),
            16 => String::from("PRI (I): "),
            17 => String::from("DUO (II): "),
            18 => String::from("TRI (III): "),
            19 => String::from("TET (IV): "),
            20 => String::from("PEN (V): "),
            _ => unreachable!(),
        },
    }
}
