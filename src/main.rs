mod bdo;
mod cli;
mod commands;
mod db;

use std::collections::HashSet;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, TokenOptions};
use commands::dev::DEVELOPER_USER_IDS;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("this should be the only call of set_global_default");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with("sqlite:bdo_items.sqlite".parse().unwrap())
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
            allow_self_messages,
            allow_bot_messages,
            case_sensitive,
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
            let prefix_options = handle_prefixes(
                prefix,
                extra_prefix,
                allow_bot_messages,
                case_sensitive,
                allow_self_messages,
            );
            run(token, prefix_options).await;
        }
    }

    Ok(())
}

async fn run(token: String, prefix_options: PrefixFrameworkOptions<Data, Error>) {
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::dev::register::devregister(),
                commands::dev::say::say(),
                commands::rng::coinflip(),
                commands::rng::dice(),
                commands::rng::ball8(),
                commands::utils::kick(),
                commands::bdo::register(),
                commands::bdo::profile(),
                commands::bdo::get_registration_queue(),
                commands::bdo::get_id(),
            ],
            prefix_options,
            owners: HashSet::from(DEVELOPER_USER_IDS),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
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

fn handle_prefixes(
    prefix: String,
    extra_prefixes: Vec<String>,
    allow_bot_messages: bool,
    case_sensitive: bool,
    allow_self_messages: bool,
) -> PrefixFrameworkOptions<Data, Error> {
    let extra_prefixes = extra_prefixes
        .into_iter()
        .map(|s| poise::Prefix::Literal(s.leak()))
        .collect();
    // PrefixFrameworkOptions { prefix: Some(prefix), additional_prefixes: , dynamic_prefix: , stripped_dynamic_prefix: , mention_as_prefix: , edit_tracker: , execute_untracked_edits: , ignore_edits_if_not_yet_responded: , execute_self_messages: , ignore_bots: ,, case_insensitive_commands:  }
    PrefixFrameworkOptions {
        prefix: Some(prefix),
        additional_prefixes: extra_prefixes,
        execute_self_messages: allow_self_messages,
        ignore_bots: !allow_bot_messages,
        case_insensitive_commands: !case_sensitive,
        // ignore_bots
        ..Default::default()
    }
}
