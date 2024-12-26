use serenity::all::Permissions;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    let message = "**Welcome to the Community! 👋**
The open source AI powered social hub. Reach out to me (in this server, not via direct message) 
if you have any questions about the project, this community, or anything else!\n\n\
    **Getting Started**\n\
    1. ⭐ Please star our project on GitHub\n\
    2. 😊 React with emojis that match your interests below\n\
    3. 👋 Introduce yourself in <#👋introductions>\n\n\
    **Getting Involved**\n\
    🐒 Available for QA Testing\n\
    🤖 Code Contributor\n\
    🙌 Art/Marketing Helper\n\
    📢 Join Community Meetings\n\
    🔔 GitHub Notifications\n\n\
    **Your Creative Background**\n\
    🦀 Rustacean\n\
    🕸️ Web Dev\n\
    🧠 AI/ML Dev/Researcher\n\
    🫖 UX Designer\n\n\
    *Click a reaction once to get the role. Click again to remove it.*";

    message.to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("welcome")
        .description("Display the welcome message with role assignments")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
