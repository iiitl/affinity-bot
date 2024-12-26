use serenity::all::{Context, Member, RoleId};
use shuttle_runtime::SecretStore;

pub async fn new_member_role_assign(
    ctx: Context,
    new_member: Member,
    secrets: SecretStore,
) -> Result<(), Box<dyn std::error::Error>> {
    let role_id = RoleId::new(
        secrets
            .get("ROLE_INTERESTED_CONTRIBUTOR")
            .ok_or("MISSING ROLE_INTERESTED_CONTRIBUTOR")?
            .parse()?,
    );

    new_member.add_role(&ctx.http, role_id).await?;

    Ok(())
}
