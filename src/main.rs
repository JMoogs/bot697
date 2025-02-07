mod cli;
mod commands;
mod db;

use std::collections::HashSet;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, TokenOptions};
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use songbird::SerenityInit;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    developers: Option<HashSet<serenity::UserId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_guilds: Option<HashSet<serenity::GuildId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefixes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_as_prefix: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_self_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_bot_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    case_sensitive: Option<bool>,
}

// User data, which is stored and accessible in all command invocations
struct Data {
    developers: HashSet<serenity::UserId>,
    developer_guilds: HashSet<serenity::GuildId>,
    config: Config,
    start_time: std::time::Instant,
    http: reqwest::Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("this should be the only call of set_global_default");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(std::env::var("DATABASE_URL").unwrap().parse().unwrap())
        .await;
    let Ok(database) = db else {
        tracing::error!("exiting as a connection to the database could not be created");
        println!("Failed to connect to the database. Exiting");
        std::process::exit(1);
    };
    db::set_database(database);

    let args = Cli::parse();

    match args.command {
        Commands::Run {
            token,
            prefix,
            extra_prefix,
            mention_as_prefix,
            allow_self_messages,
            allow_bot_messages,
            case_sensitive,
            config_file,
            developer_id,
            developer_guild,
        } => {
            let token = get_token(token);
            let token = match token {
                Ok(t) => t,
                Err(_) => {
                    tracing::error!("exiting as a token could not be read");
                    println!("Failed to find a token.\
                        \nEither set the environment variable DISCORD_TOKEN or use one of the following arguments:\
                        \n\t--token to directly read the token\
                        \n\t--token_var to read a different environment variable\
                        \n\t--token_file to read the token from a file\
                        \nTerminating");
                    std::process::exit(1);
                }
            };

            let mut config: Config = match config_file {
                Some(f) => match std::fs::read_to_string(f) {
                    Err(e) => {
                        tracing::error!("failed to read config file: {}", e);
                        println!("Failed to read the config file. Exiting");
                        std::process::exit(1);
                    }
                    Ok(s) => match serde_json::from_str(&s) {
                        Err(e) => {
                            tracing::error!("failed to parse config file: {}", e);
                            println!("Failed to parse the config file. Exiting");
                            std::process::exit(1);
                        }
                        Ok(c) => c,
                    },
                },
                None => Config::default(),
            };

            merge_config_cli_args(
                &mut config,
                prefix,
                extra_prefix,
                mention_as_prefix,
                allow_bot_messages,
                case_sensitive,
                allow_self_messages,
                developer_id,
                developer_guild,
            );

            let prefix_options = handle_prefixes(&config);
            run(token, prefix_options, &config).await;
        }
    }

    Ok(())
}

async fn run(token: String, prefix_options: PrefixFrameworkOptions<Data, Error>, config: &Config) {
    let intents = serenity::GatewayIntents::all();

    let developer_user_ids = config.developers.clone().unwrap_or_default();
    let developer_guild_ids = config.developer_guilds.clone().unwrap_or_default();

    let conf = config.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::dev::register::devregister(),
                commands::dev::say::say(),
                commands::dev::dumpconfig(),
                commands::rng::coinflip(),
                commands::rng::dice(),
                commands::rng::ball8(),
                commands::utils::kick(),
                commands::utils::avatar(),
                commands::utils::uptime(),
                commands::utils::userinfo(),
                commands::voice::join(),
                commands::voice::leave(),
                commands::voice::mute(),
                commands::voice::unmute(),
                commands::voice::deafen(),
                commands::voice::undeafen(),
                commands::voice::play(),
                commands::voice::skip(),
                commands::voice::stop(),
            ],
            prefix_options,
            owners: developer_user_ids.clone(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    developers: developer_user_ids,
                    developer_guilds: developer_guild_ids,
                    config: conf,
                    start_time: std::time::Instant::now(),
                    http: reqwest::Client::new(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}

fn get_token(options: TokenOptions) -> Result<String, ()> {
    if let Some(t) = options.token {
        tracing::info!("a token was provided directly to the command, it will be used");
        return Ok(t);
    } else if let Some(t) = options.token_var {
        tracing::info!("reading token from environment variable: {}", t);
        match std::env::var(t) {
            Ok(token) => Ok(token),
            Err(e) => {
                tracing::error!("failed to read token from environment variable: {}", e);
                return Err(());
            }
        }
    } else if let Some(t) = options.token_file {
        tracing::info!("reading token from file: {}", t);
        let t = std::fs::read_to_string(t);

        match t {
            Ok(token) => Ok(token),
            Err(e) => {
                tracing::error!("failed to read token from file: {}", e);
                return Err(());
            }
        }
    } else {
        match std::env::var("DISCORD_TOKEN") {
            Ok(token) => Ok(token),
            Err(e) => {
                tracing::error!("failed to read token from DISCORD_TOKEN: {}", e);
                return Err(());
            }
        }
    }
}

fn merge_config_cli_args(
    config: &mut Config,
    prefix: Option<String>,
    extra_prefixes: Vec<String>,
    mention_as_prefix: bool,
    allow_bot_messages: bool,
    case_sensitive: bool,
    allow_self_messages: bool,
    developer_id: Vec<String>,
    developer_guild: Vec<String>,
) {
    // Create a list of all prefixes, with the main prefix first
    let mut prefixes = Vec::new();
    if let Some(p) = prefix.clone() {
        prefixes.push(p);
    }
    if let Some(p) = config.prefixes.clone() {
        prefixes.extend(p);
    }
    prefixes.extend(extra_prefixes);
    if prefix.is_some() {
        config.prefixes = Some(vec![prefix.unwrap()]);
    }

    config.prefixes = Some(prefixes);

    // CLI priority
    if mention_as_prefix {
        config.mention_as_prefix = Some(true);
    }

    if allow_bot_messages {
        config.allow_bot_messages = Some(true);
    }

    if case_sensitive {
        config.case_sensitive = Some(true);
    }

    if allow_self_messages {
        config.allow_self_messages = Some(true);
    }

    // Merge developer IDs
    let mut dev_ids = config.developers.clone().unwrap_or_default();

    for id in developer_id {
        match id.parse::<u64>() {
            Ok(id) => {
                dev_ids.insert(serenity::UserId::new(id));
            }
            Err(e) => {
                tracing::error!("failed to parse developer ID: {}, skipping", e);
            }
        }
    }
    config.developers = Some(dev_ids);

    // Merge developer guilds
    let mut dev_guilds = config.developer_guilds.clone().unwrap_or_default();

    for id in developer_guild {
        match id.parse::<u64>() {
            Ok(id) => {
                dev_guilds.insert(serenity::GuildId::new(id));
            }
            Err(e) => {
                tracing::error!("failed to parse developer guild ID: {}, skipping", e);
            }
        }
    }

    config.developer_guilds = Some(dev_guilds);
}

fn handle_prefixes(
    config: &Config,
    // prefix: Option<String>,
    // extra_prefixes: Vec<String>,
    // mention_as_prefix: bool,
    // allow_bot_messages: bool,
    // case_sensitive: bool,
    // allow_self_messages: bool,
) -> PrefixFrameworkOptions<Data, Error> {
    // Ensure we have at least one prefix
    if config.prefixes.is_none() {
        tracing::error!("exiting as no prefix was provided");
        println!(
            "No prefix was provided.\
            \nEither set the --prefix argument or provide a config file with the prefix field.\
            \nTerminating"
        );
        std::process::exit(1);
    }

    let main_prefix = config
        .prefixes
        .clone()
        .expect("we checked there is at least 1 prefix")
        .first()
        .unwrap()
        .clone();
    let extra_prefixes = config
        .prefixes
        .clone()
        .expect("we checked there is at least 1 prefix")
        .into_iter()
        .skip(1)
        .map(|s| poise::Prefix::Literal(s.leak()))
        .collect();

    // Any of the following being true means they were explicitly set by the user in CLI, so we can use them as is. If they're false we check the config
    PrefixFrameworkOptions {
        prefix: Some(main_prefix),
        additional_prefixes: extra_prefixes,
        execute_self_messages: config.allow_self_messages.unwrap_or(false),
        ignore_bots: !config.allow_bot_messages.unwrap_or(false),
        case_insensitive_commands: !config.case_sensitive.unwrap_or(false),
        mention_as_prefix: config.mention_as_prefix.unwrap_or(false),
        ..Default::default()
    }
}
