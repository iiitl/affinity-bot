use std::error::Error;
use std::time::Duration;

use anyhow::Context as _;
use events::self_role_assign::self_role_assign;
use serenity::all::{
    ChannelId, Command, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
    Member, MessageId, Reaction, ReactionType, User,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{all::GuildId, async_trait};
use shuttle_runtime::SecretStore;
use tracing::{error, info};
mod commands;
mod events;
// Modify Bot struct to hold SecretStore
struct Bot {
    secrets: SecretStore,
}

// Add constructor for Bot
impl Bot {
    pub fn new(secrets: SecretStore) -> Self {
        Self { secrets }
    }
    async fn reaction_add_internal(
        &self,
        ctx: Context,
        reaction: Reaction,
    ) -> Result<(), Box<dyn Error>> {
        // Parse the role_message_id
        let role_message_id = match self
            .secrets
            .get("ROLE_EMOJI_MESSAGE_ID")
            .ok_or("Missing ROLE_EMOJI_MESSAGE_ID")?
            .parse()
        {
            Ok(id) => MessageId::new(id),
            Err(e) => {
                error!("Failed to parse ROLE_EMOJI_MESSAGE_ID: {:?}", e);
                return Err(Box::new(e));
            }
        };

        println!("hello i am here");

        // Check if the reaction matches the role message
        if reaction.message_id == role_message_id {
            self_role_assign(ctx, reaction, self.secrets.clone()).await?;
        }

        Ok(())
    }

    async fn new_members(&self, ctx: Context, new_member: Member) -> Result<(), Box<dyn Error>> {
        events::onboarding_role::new_member_role_assign(ctx, new_member, self.secrets.clone())
            .await?;

        Ok(())
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
        let welcome_channel_id = match self
            .secrets
            .get("WELCOME_CHANNEL_ID")
            .expect("Set WELCOME_CHANNEL_ID")
            .parse()
        {
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
        if let Err(e) = self.new_members(ctx, new_member).await {
            error!("Error handling new_members: {:?}", e);
        }
    }
    async fn guild_member_removal(&self,ctx:Context,_: GuildId, user: User, _: Option<Member>)
    {
        let welcome_channel_id = match self
            .secrets
            .get("WELCOME_CHANNEL_ID")
            .expect("Set WELCOME_CHANNEL_ID")
            .parse()
        {
            Ok(id) => ChannelId::new(id),
            Err(e) => {
                error!("Failed to parse WELCOME_CHANNEL_ID: {:?}", e);
                return;
            }
        };

        let goodbye_message = format!("Will Miss You {}! 👋", user.mention());
        if let Err(e) = welcome_channel_id.say(&ctx.http, goodbye_message).await {
            error!("Error sending welcome message: {:?}", e);
        }

    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(e) = self.reaction_add_internal(ctx, reaction).await {
            error!("Error handling reaction_add: {:?}", e);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                "welcome" => {
                    // Handle the welcome command
                    let message = commands::welcome_message::run(&command.data.options());
                    let data = CreateInteractionResponseMessage::new().content(message);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        error!("Cannot respond to welcome command: {why}");
                    } else {
                        // Get the message after sending it
                        if let Ok(message) = command.get_response(&ctx.http).await {
                            // Add reactions one by one
                            let reactions = [
                                // Backgrounds
                                "🦀", "🕸️", "🧠", "🫖",
                            ];

                            for reaction in reactions {
                                // Add each reaction with a small delay to avoid rate limiting
                                if let Err(e) = message
                                    .react(&ctx.http, ReactionType::Unicode(reaction.to_string()))
                                    .await
                                {
                                    error!("Error adding reaction {}: {:?}", reaction, e);
                                }
                                // Small delay between reactions to avoid rate limits
                                tokio::time::sleep(Duration::from_millis(300)).await;
                            }
                        }
                    }
                    None
                }
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
                .expect("Failed to parse GUILD_TOKEN"),
        );

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::ping::register(),
                    commands::id::register(),
                    commands::welcome_message::register(),
                ],
            )
            .await;

        let _ = Command::create_global_command(&ctx.http, commands::wonderful_command::register())
            .await;
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    // Pass secrets to Bot constructor
    let client = Client::builder(&token, intents)
        .event_handler(Bot::new(secrets))
        .await
        .expect("Err creating client");

    Ok(client.into())
}
