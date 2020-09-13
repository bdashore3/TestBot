use serenity::{client::Context, model::channel::Message, framework::standard::CommandResult, framework::standard::macros::command};


#[command]
#[required_permissions("ADMINISTRATOR")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Ping!").await?;

    Ok(())
}
