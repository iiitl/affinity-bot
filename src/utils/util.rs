use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

pub enum Response {
    Text(String),
    Embed(Box<CreateEmbed>),
}
impl From<String> for Response {
    fn from(content: String) -> Self {
        Response::Text(content)
    }
}

impl From<CreateEmbed> for Response {
    fn from(embed: CreateEmbed) -> Self {
        Response::Embed(Box::new(embed))
    }
}

pub async fn create_response(
    ctx: &Context,
    command: &CommandInteraction,
    response: impl Into<Response>,
) {
    let response = response.into();
    let builder = CreateInteractionResponse::Message(match response {
        Response::Text(content) => CreateInteractionResponseMessage::new().content(content),
        Response::Embed(embed) => CreateInteractionResponseMessage::new().embed(*embed),
    });

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}
