use teloxide::{
    prelude::*, 
    utils::command::BotCommands, 
    types::*, payloads::GetChat,
};

use std::env;

use reqwest::*;

#[tokio::main]
async fn main() {

    env::set_var("TELOXIDE_TOKEN", "6402266107:AAFSAZN2r1fxJRrvvCw4wexI1b9wKJwOYgM");

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let filter_channel_post_handler = Update::filter_channel_post().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            
            // let chat = bot.get_chat(q.chat.id).send().await.unwrap();

            // q.chat.bio();
            // q.chat.description();
            // q.chat.first_name();
            // q.chat.last_name();
            // q.chat.username();
            // q.chat.title();

            println!("channel post {} {}" , q.text().unwrap(), q.chat.title().unwrap());

            respond(())
        })
    );

    let filter_message_handler = Update::filter_message().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            println!("message {} {}" , q.text().unwrap(), q.chat.title().unwrap());

            respond(())
        })
    );

    let filter_inline_query_handler = Update::filter_inline_query().branch(
        dptree::endpoint(|bot: Bot, q: InlineQuery| async move {
            println!("inline query {}" , q.query);

            respond(())
        })
    );

    let filter_callback_query_handler = Update::filter_callback_query().branch(
        dptree::endpoint(|bot: Bot, q: CallbackQuery| async move {
            println!("callback query");

            respond(())
        })
    );

    let filter_chosen_inline_result_handler = Update::filter_chosen_inline_result().branch(
        dptree::endpoint(|bot: Bot, q: ChosenInlineResult| async move {
            println!("chosen inline result");

            respond(())
        })
    );

    let filter_edited_channel_post_handler = Update::filter_edited_channel_post().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            println!("edited channel post");

            respond(())
        })
    );

    let filter_edited_message_handler = Update::filter_edited_message().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            println!("edited message");

            respond(())
        })
    );

    let filter_poll_handler = Update::filter_poll().branch(
        dptree::endpoint(|bot: Bot, q: Poll| async move {
            println!("poll");

            respond(())
        })
    );

    let filter_pre_checkout_query_handler = Update::filter_pre_checkout_query().branch(
        dptree::endpoint(|bot: Bot, q: PreCheckoutQuery| async move {
            println!("pre checkout query");

            respond(())
        })
    );

    let filter_shipping_query_handler = Update::filter_shipping_query().branch(
        dptree::endpoint(|bot: Bot, q: ShippingQuery| async move {
            println!("shipping query");

            respond(())
        })
    );

    Dispatcher::builder(bot.clone(), filter_edited_channel_post_handler)
        .enable_ctrlc_handler().build()
        .dispatch().await;

    // Dispatcher::builder(bot.clone(), filter_channel_post_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_message_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_inline_query_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_callback_query_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_chosen_inline_result_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_edited_message_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_poll_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_pre_checkout_query_handler).enable_ctrlc_handler().build().dispatch().await;
    // Dispatcher::builder(bot.clone(), filter_shipping_query_handler).enable_ctrlc_handler().build().dispatch().await;


    // let handler = Update::filter_inline_query().branch(dptree::endpoint(
    //     |bot: Bot, q: InlineQuery| async move {
    //         // First, create your actual response
    //         let google_search = InlineQueryResultArticle::new(
    //             // Each item needs a unique ID, as well as the response container for the
    //             // items. These can be whatever, as long as they don't
    //             // conflict.
    //             "01".to_string(),
    //             // What the user will actually see
    //             "Google Search",
    //             // What message will be sent when clicked/tapped
    //             InputMessageContent::Text(InputMessageContentText::new(format!(
    //                 "https://www.google.com/search?q={}",
    //                 q.query,
    //             ))),
    //         );
    //         // While constructing them from the struct itself is possible, it is preferred
    //         // to use the builder pattern if you wish to add more
    //         // information to your result. Please refer to the documentation
    //         // for more detailed information about each field. https://docs.rs/teloxide/latest/teloxide/types/struct.InlineQueryResultArticle.html
    //         let ddg_search = InlineQueryResultArticle::new(
    //             "02".to_string(),
    //             "DuckDuckGo Search".to_string(),
    //             InputMessageContent::Text(InputMessageContentText::new(format!(
    //                 "https://duckduckgo.com/?q={}",
    //                 q.query
    //             ))),
    //         )
    //         .description("DuckDuckGo Search")
    //         .thumb_url("https://duckduckgo.com/assets/logo_header.v108.png".parse().unwrap())
    //         .url("https://duckduckgo.com/about".parse().unwrap()); // Note: This is the url that will open if they click the thumbnail

    //         let results = vec![
    //             InlineQueryResult::Article(google_search),
    //             InlineQueryResult::Article(ddg_search),
    //         ];

    //         // Send it off! One thing to note -- the ID we use here must be of the query
    //         // we're responding to.
    //         let response = bot.answer_inline_query(&q.id, results).send().await;
    //         if let Err(err) = response {
    //             log::error!("Error in handler: {:?}", err);
    //         }
    //         respond(())
    //     },
    // ));

    // Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    //Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "return a test button.")]
    Navigation,

}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {

    println!("{}", msg.chat.id);


    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
        Command::Navigation => {
            bot.send_message(msg.chat.id, "This is a fast navigation.")
            .reply_markup(
                InlineKeyboardMarkup::default()
                .append_row(vec![
                    InlineKeyboardButton{ 
                        text: String::from("Pay"), 
                        kind: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(String::from("ClubCurrentChat"))
                    }
                ])
                .append_row(vec![
                    InlineKeyboardButton{ 
                        text: String::from("Club0"), 
                        kind: InlineKeyboardButtonKind::SwitchInlineQuery(String::from("Club0"))
                    },
                    InlineKeyboardButton{ 
                        text: String::from("Green Sea"), 
                        kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
                    },
                    InlineKeyboardButton{ 
                        text: String::from("Forward"), 
                        kind: InlineKeyboardButtonKind::SwitchInlineQuery(String::from("Club1"))
                    },
                ])
            ).await?
        }
    };

    Ok(())
}