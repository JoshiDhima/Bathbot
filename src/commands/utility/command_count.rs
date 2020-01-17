use crate::CommandCounter;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use std::fmt::Write;

#[command]
#[bucket = "two_per_thirty_cooldown"]
fn commands(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();
    let data = ctx.data.read();
    let counter = data
        .get::<CommandCounter>()
        .expect("Expected CommandCounter in ShareMap.");
    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name = k, amount = v);
    }
    if let Err(why) = msg.channel_id.say(&ctx.http, &contents) {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}