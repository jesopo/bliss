use super::error::Command as CommandError;
use reqwest::Url;
use serenity::model::channel::AttachmentType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};

pub(crate) async fn run(
    options: &[CommandDataOption],
) -> Result<(String, Option<AttachmentType>), CommandError> {
    let mut url = Url::parse("https://bsky.social/").unwrap();

    if let Some(CommandDataOptionValue::String(path)) =
        options.get(0).and_then(|o| o.resolved.as_ref())
    {
        let mut path = path.clone();

        if let Some(path_stripped) = path.strip_prefix("https://bsky.social/xrpc/") {
            path = path_stripped.to_string();
        }

        if let Some((path, query)) = path.split_once('?') {
            url.set_path(&format!("xrpc/{path}"));
            url.set_query(Some(query));
        } else {
            url.set_path(&format!("xrpc/{path}"));
        }

        let response = reqwest::Client::new()
            .get(url.clone())
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        let json = serde_json::to_string_pretty(&response).unwrap();

        let mut hasher = DefaultHasher::new();
        json.hash(&mut hasher);

        Ok((
            format!("> {url}\n"),
            Some(AttachmentType::Bytes {
                filename: format!("{}.json", hasher.finish()),
                data: json.into_bytes().into(),
            }),
        ))
    } else {
        Err(CommandError::MissingArgument)
    }
}
