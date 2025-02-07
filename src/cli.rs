use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "697's Discord Bot")]
#[command(propagate_version = true)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the bot
    Run {
        #[command(flatten)]
        token: TokenOptions,

        /// Load the configuration from a json file
        ///
        /// In general, CLI arguments take precedence over configuration file values
        ///
        /// Where possible, configuration file values will be used in addition to CLI arguments
        #[arg(long)]
        config_file: Option<String>,

        /// Set a prefix for bot commands
        #[arg(long, short, required_unless_present = "config_file")]
        prefix: Option<String>,

        /// Set additional prefixes
        #[arg(long)]
        extra_prefix: Vec<String>,

        /// Count mentions of the bot as a valid prefix
        #[arg(long)]
        mention_as_prefix: bool,

        /// Allow the bot to trigger commands from its own messages. The bot must allow bot messages for this to work.
        #[arg(long)]
        allow_self_messages: bool,

        /// Allow other bots to trigger command
        #[arg(long)]
        allow_bot_messages: bool,

        /// Make commands case sensitive
        #[arg(long)]
        case_sensitive: bool,

        // Allow users with given IDs to run developer commands
        #[arg(long)]
        developer_id: Vec<String>,

        // Allow developer commands to run in the given guilds
        #[arg(long)]
        developer_guild: Vec<String>,
    },
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct TokenOptions {
    /// Run the bot using the provided token
    #[arg(long, short)]
    pub token: Option<String>,

    /// Read the token from a different environment variable
    ///
    /// The default variable is DISCORD_TOKEN
    #[arg(long)]
    pub token_var: Option<String>,

    /// Read the token from a file
    #[arg(long)]
    pub token_file: Option<String>,
}
