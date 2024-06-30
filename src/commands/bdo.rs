use crate::{
    bdo::{
        calculate_tax_rate, create_user_context, format_number, get_enhancement_prefix,
        get_waitlist, items::get_matching_items_fuzzy, GearType,
    },
    db::get_database,
    Context, Error,
};
use bdo_enhancement_calc::requests::Region;
use poise::{serenity_prelude as serenity, ChoiceParameter};

#[derive(poise::ChoiceParameter)]
enum CronCost {
    // 3000000
    #[name = "Vendor Pricing"]
    Vendor,
    // 2185033
    #[name = "Outfit Pricing"]
    Outfit,
    #[name = "Free"]
    Free,
}

#[derive(poise::ChoiceParameter)]
enum BDORegion {
    #[name = "Europe"]
    Eu,
    #[name = "North America"]
    Na,
    #[name = "Southeast Asia"]
    Sea,
    #[name = "Korea"]
    Kr,
    #[name = "Russia"]
    Ru,
    #[name = "Japan"]
    Jp,
    #[name = "EU Console"]
    ConsoleEu,
    #[name = "NA Console"]
    ConsoleNa,
}

impl BDORegion {
    pub fn get_inner_region(&self) -> Region {
        match self {
            BDORegion::Eu => Region::Eu,
            BDORegion::Na => Region::Na,
            BDORegion::Sea => Region::Sea,
            BDORegion::Kr => Region::Kr,
            BDORegion::Ru => Region::Ru,
            BDORegion::Jp => Region::Jp,
            BDORegion::ConsoleEu => Region::ConsoleEu,
            BDORegion::ConsoleNa => Region::ConsoleNa,
        }
    }
}

/// Register your BDO info
#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "the region you play in"] region: BDORegion,
    #[description = "whether or not you are using a value pack"] value_pack: bool,
    #[description = "the amount of family fame you have"] family_fame: u32,
    #[description = "whether or not you have the rich merchant's ring"] merchant_ring: bool,
    #[description = "the source (and therefore price) of your cron stones"] crons: CronCost,
) -> Result<(), Error> {
    let author = ctx.author();
    let author_id = author.id.get() as i64;

    let reg = region.get_inner_region() as i64;
    let vp = if value_pack { 1 } else { 0 };
    let mr = if merchant_ring { 1 } else { 0 };
    let ff = family_fame as i64;
    let cron_price = match crons {
        CronCost::Vendor => 3_000_000,
        CronCost::Outfit => 2_185_033,
        CronCost::Free => 0,
    };

    let db = get_database();
    sqlx::query!(
        "INSERT OR REPLACE INTO Users
        VALUES (?, ?, ?, ?, ?, ?)
        ",
        author_id,
        ff,
        vp,
        mr,
        cron_price,
        reg
    )
    .execute(db)
    .await?;

    let response = ctx
        .send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title(format!("Successfully registered"))
                    .thumbnail(author.avatar_url().unwrap_or_else(|| {
                        String::from("https://cdn.discordapp.com/embed/avatars/0.png")
                    }))
                    .timestamp(serenity::Timestamp::now())
                    .colour((255, 0, 0))
                    .field("Region:", region.name(), false)
                    .field("Value Pack:", if value_pack { "Yes" } else { "No" }, false)
                    .field("Family Fame:", family_fame.to_string(), false)
                    .field(
                        "Merchant Ring:",
                        if merchant_ring { "Yes" } else { "No" },
                        false,
                    )
                    .field("Cron Price:", format!("{} silver", cron_price), false),
            ),
        )
        .await;

    match response {
        Ok(_) => (),
        Err(e) => {
            tracing::warn!(
                "Failed to notify the user that they registered when running 'register': {}",
                e
            );
        }
    }

    return Ok(());
}

/// Displays your profile
///
/// Displays your profile
#[poise::command(slash_command)]
pub async fn profile(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();
    let author_id = author.id.get() as i64;

    let db = get_database();
    let res = sqlx::query!(
        "SELECT * FROM Users
        WHERE discord_id = ?",
        author_id
    )
    .fetch_one(db)
    .await;

    match res {
        Ok(r) => {
            ctx.send(
                poise::CreateReply::default().embed(
                    serenity::CreateEmbed::new()
                        .title(format!(
                            "{}'s Profile",
                            author
                                .global_name
                                .clone()
                                .unwrap_or_else(|| author.name.clone())
                        ))
                        .thumbnail(author.avatar_url().unwrap_or_else(|| {
                            String::from("https://cdn.discordapp.com/embed/avatars/0.png")
                        }))
                        .timestamp(serenity::Timestamp::now())
                        .colour((255, 0, 0))
                        .field(
                            "Region:",
                            Region::from_i64(r.region)
                                .expect("Db should always be valid")
                                .get_display_name(),
                            false,
                        )
                        .field(
                            "Value Pack:",
                            if r.value_pack == 0 { "No" } else { "Yes" },
                            false,
                        )
                        .field(
                            "Merchant Ring:",
                            if r.merchant_ring == 0 { "No" } else { "Yes" },
                            false,
                        )
                        .field("Family Fame:", r.family_fame.to_string(), false)
                        .field("Cron Cost:", format!("{} silver", r.cron_cost), false)
                        .field(
                            "Tax:",
                            format!(
                                "You are given {:.3}% of the value of the item.",
                                100.0
                                    * calculate_tax_rate(
                                        r.value_pack != 0,
                                        r.family_fame as u64,
                                        r.merchant_ring != 0
                                    )
                            ),
                            false,
                        ),
                ),
            )
            .await?;
        }
        Err(_) => {
            ctx.say("Profile not found. Please register using /register first.")
                .await?;
        }
    }

    Ok(())
}

/// Lists the registration queue for your region
///
/// Lists the registration queue for your region
#[poise::command(prefix_command, slash_command)]
pub async fn get_registration_queue(ctx: Context<'_>) -> Result<(), Error> {
    let context = create_user_context(ctx.author().id.get()).await;
    match context {
        Ok(c) => {
            let l = get_waitlist(&c).await?;
            if l.is_empty() {
                ctx.say("No items are currently in the registration queue. If you believe this is a bug, message d697").await?;
            } else {
                let mut field_entries = Vec::with_capacity(l.len());
                for item in l {
                    // Accessories
                    let name = if item.main_category == 20 {
                        let prefix =
                            get_enhancement_prefix(item.enhancement_level, GearType::Accessory);
                        format!("{}{}", prefix, item.name)
                    } else if [719899, 719955, 719898, 719897, 719900, 719956]
                        .contains(&item.item_id)
                    {
                        let prefix =
                            get_enhancement_prefix(item.enhancement_level, GearType::Accessory);
                        let n = format!("{}{}", prefix, item.name);
                        // Specify which gloves/boots as required
                        // DR
                        if [719899, 719900].contains(&item.item_id) {
                            format!("{} (Damage Reduction)", n)
                        // Eva
                        } else if [719955, 719956].contains(&item.item_id) {
                            format!("{} (Evasion)", n)
                        } else {
                            n
                        }
                    } else {
                        let prefix = get_enhancement_prefix(item.enhancement_level, GearType::Gear);
                        format!("{}{}", prefix, item.name)
                    };

                    field_entries.push((
                        name,
                        format!(
                            "{} silver\n Live: <t:{}:R>",
                            format_number(item.price),
                            item.list_time / 1000
                        ),
                        false,
                    ));
                }

                // Only 25 fields are allowed so we need to send multiple messages for more
                if field_entries.len() <= 25 {
                    ctx.send(
                        poise::CreateReply::default().embed(
                            serenity::CreateEmbed::new()
                                .title(format!(
                                    "{} Registration Queue",
                                    c.region.get_display_name()
                                ))
                                .timestamp(serenity::Timestamp::now())
                                .fields(field_entries),
                        ),
                    )
                    .await?;
                } else {
                    for (i, chunk) in field_entries.chunks(25).enumerate() {
                        ctx.send(
                            poise::CreateReply::default().embed(
                                serenity::CreateEmbed::new()
                                    .title(format!(
                                        "{} Registration Queue Part {}",
                                        c.region.get_display_name(),
                                        i + 1
                                    ))
                                    .timestamp(serenity::Timestamp::now())
                                    .colour((255, 0, 0))
                                    .fields(chunk.to_vec()),
                            ),
                        )
                        .await?;
                    }
                }
            }
        }
        Err(_) => {
            ctx.say("Please register with /register before using this command.")
                .await?;
        }
    }

    Ok(())
}

/// Find the ID of an item
#[poise::command(slash_command, prefix_command)]
pub async fn get_id(
    ctx: Context<'_>,
    #[description = "the search term"]
    #[rest]
    search_term: String,
) -> Result<(), Error> {
    let items = get_matching_items_fuzzy(search_term.clone(), 10);
    let items: Vec<(String, String, bool)> = items
        .into_iter()
        .map(|(name, id)| (name, format!("Item ID: {}", id), false))
        .collect();
    if !items.is_empty() {
        ctx.send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title(format!("Best matches for \"{}\"", search_term))
                    .colour((255, 0, 0))
                    .timestamp(serenity::Timestamp::now())
                    .fields(items),
            ),
        )
        .await?;
    } else {
        ctx.say(format!("No matches found for: \"{}\"", search_term))
            .await?;
    }
    Ok(())
}
