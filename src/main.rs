#[cfg(debug_assertions)]
use dotenvy::dotenv;

use regex::Regex;
use serenity::all::{EditMessage, MessageBuilder};
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        if !msg.author.bot {
            let regex = Regex::new(r"twitter\.com").unwrap();
            let mut new_embeds: Vec<String> = Vec::new();
            println!("{:?}", msg.embeds);
            for embed in msg.embeds.clone() {
                if let Some(url) = embed.url
                    && regex.is_match(url.as_str())
                {
                    println!("{url}");
                    new_embeds.push(format!(
                        "[⠀]({})",
                        regex.replace(url.as_str(), "fxtwitter.com")
                    ));
                }
            }

            if !new_embeds.is_empty() {
                let mut msg_builder = MessageBuilder::new();
                for embed in new_embeds {
                    msg_builder.push(embed);
                }

                msg.reply(&ctx.http, msg_builder.build()).await.unwrap();

                msg.edit(&ctx.http, EditMessage::new().suppress_embeds(true))
                    .await
                    .unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
