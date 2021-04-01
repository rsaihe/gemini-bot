use serenity::model::prelude::*;
use serenity::utils;

/// Parse a UserId from text.
pub async fn parse_user_id(value: &str, guild: Option<&Guild>) -> Option<UserId> {
    // Attempt to parse a mention.
    if let Some(id) = utils::parse_username(&value) {
        return Some(id.into());
    }

    // Attempt to parse a numerical ID.
    if let Ok(id) = value.parse::<u64>() {
        return Some(id.into());
    }

    // Attempt to find a guild member with an exact name match.
    if let Some(member) = guild?.member_named(&value) {
        return Some(member.user.id);
    }

    // Attempt to find a guild member with the name matching a substring.
    guild?
        .members_containing(&value, false, true)
        .await
        .first()
        .map(|(m, _)| m.user.id)
}
