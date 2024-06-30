use crate::serenity;

pub mod register;
pub mod say;

pub const DEVELOPER_USER_IDS: [serenity::UserId; 2] = [
    serenity::UserId::new(227446222632255489),
    serenity::UserId::new(842497297770348615),
];

pub const DEVELOPER_GUILDS: [serenity::GuildId; 3] = [
    serenity::GuildId::new(582117329337450499),
    serenity::GuildId::new(773889931336482836),
    serenity::GuildId::new(773967376890462238),
    // serenity::GuildId::new(627827225797853185),
];
