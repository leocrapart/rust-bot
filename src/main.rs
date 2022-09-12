use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(plant, cut, leaderboard)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn plant(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "1 tree has been planted").await?;
    Ok(())
}

#[command]
async fn cut(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "1 tree has been cut").await?;
    Ok(())
}


#[command]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "1 leoo#3446             (**155**)\n2 EnzoLeDebilo#6159 (**19**)").await?;
    Ok(())
}


