mod command_bleval;
mod error;

use self::error::Command as CommandError;
use async_trait::async_trait;
use clap::Parser;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(index = 1)]
    token: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let (header, body) = match command.data.name.as_str() {
                "bleval" => self::command_bleval::run(&command.data.options).await,
                _ => Err(CommandError::UnknownCommand),
            }
            .unwrap_or_else(|e| (format!("{e:?}"), None));

            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            let mut message = message.content(header);
                            if let Some(body) = body {
                                message = message.add_file(body);
                            }
                            message
                        })
                })
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let mut client = Client::builder(args.token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(e) = client.start().await {
        println!("client error: {e:?}");
    }
}
