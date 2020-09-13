mod commands;

use std::{collections::HashSet, env};
use serenity::{http::Http, framework::StandardFramework, framework::standard::macros::{command, group}, client::Context, model::channel::Message, framework::standard::DispatchError, framework::standard::macros::hook, Client};
use crate::commands::ping::*;

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(pong)]
struct Other;

#[hook]
async fn dispatch_error(_: &Context, _: &Message, error: DispatchError) {
    println!("Dispatch error!: {:?}", error);
}

#[hook]
async fn unrecognised_command_hook(_: &Context, msg: &Message, unrecognised_command_name: &str) {
    println!("A user named {:?} tried to executute an unknown command: {}",
        msg.author.name, unrecognised_command_name
    );
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect(
        "Expected a token in the environment",
    );

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
    .configure(|c| c
        .prefix("~")
        .owners(owners)
    )

    .on_dispatch_error(dispatch_error)
    .unrecognised_command(unrecognised_command_hook)

    // Error occurs here, try getting rid of the OTHER_GROUP and its associated commands
    .group(&GENERAL_GROUP)
    .group(&OTHER_GROUP);

    let mut client = Client::new(&token)
    .framework(framework)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        eprintln!("Client error: {:?}", why);
    }
}
