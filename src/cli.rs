use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "D697's Discord Bot")]
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

        /// Set a prefix for bot commands
        #[arg(long, short, default_value_t = String::from(","))]
        prefix: String,

        /// Set additional prefixes
        #[arg(long)]
        extra_prefix: Vec<String>,

        /// Allow the bot to trigger commands from its own messages. The bot must allow bot messages for this to work.
        #[arg(long)]
        allow_self_messages: bool,

        /// Allow other bots to trigger command
        #[arg(long)]
        allow_bot_messages: bool,

        /// Make commands case sensitive
        #[arg(long)]
        case_sensitive: bool,
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
