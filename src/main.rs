use anyhow::Context as _;
use serenity::all::{ChannelId, Command, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Member};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{all::GuildId, async_trait};
use shuttle_runtime::SecretStore;
use tracing::{error, info};
mod commands;
// Modify Bot struct to hold SecretStore
struct Bot {
    secrets: SecretStore,
}

// Add constructor for Bot
impl Bot {
    pub fn new(secrets: SecretStore) -> Self {
        Self { secrets }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let welcome_channel_id = match self.secrets.get("WELCOME_CHANNEL_ID").expect("Set WELCOME_CHANNEL_ID").parse() {
            Ok(id) => ChannelId::new(id),
            Err(e) => {
                error!("Failed to parse WELCOME_CHANNEL_ID: {:?}", e);
                return;
            }
        };

        let welcome_message = format!("Welcome to the server, {}! 👋", new_member.mention());

        if let Err(e) = welcome_channel_id.say(&ctx.http, welcome_message).await {
            error!("Error sending welcome message: {:?}", e);
        }
    }
    


    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Now we can access secrets through self.secrets
        let guild_id = GuildId::new(
            self.secrets
                .get("GUILD_TOKEN")
                .context("secret not set")
                .unwrap()
                .parse()
                .expect("Failed to parse GUILD_TOKEN")
        );

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::ping::register(),
                    commands::id::register(),
                    commands::attachmentinput::register(),
                    // commands::modal::register(),
                ],
            )
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;

    // Pass secrets to Bot constructor
    let client = Client::builder(&token, intents)
        .event_handler(Bot::new(secrets))
        .await
        .expect("Err creating client");

    Ok(client.into())
}
