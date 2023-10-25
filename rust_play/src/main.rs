use teloxide::{
    prelude::*, 
    utils::command::BotCommands, 
    types::*
};

use std::{env, collections::HashMap};

#[tokio::main]
async fn main() {

    env::set_var("TELOXIDE_TOKEN", "6402266107:AAFSAZN2r1fxJRrvvCw4wexI1b9wKJwOYgM");

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let filter_message_handler = Update::filter_message().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            
            println!("message {}", q.text().unwrap_or(""));

            if q.text().unwrap_or("") == "醋鸡" {
                bot.delete_message(q.chat.id, q.id).await.unwrap();
                
                let new_message = bot.send_message(q.chat.id, "醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡")
                .reply_markup(
                    InlineKeyboardMarkup::default()
                    .append_row(vec![
                        InlineKeyboardButton{ 
                            text: String::from("浦东新区"), 
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+W4Yki78fMccyM2Y1").unwrap())
                        }
                    ])
                    .append_row(vec![
                        InlineKeyboardButton{
                            text: String::from("碧之海 崂山路"),
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
                        },
                        InlineKeyboardButton{
                            text: String::from("虹之间 沪南路"),
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
                        }
                    ])
                    .append_row(vec![
                        InlineKeyboardButton{
                            text: String::from("心愿养生 五莲路"),
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
                        }
                    ])
                ).await.unwrap();
                
                bot.pin_chat_message(q.chat.id, new_message.id).await.unwrap();
            }

            let full_name = match q.from() {
                Some(user) => user.full_name(),
                None => String::from("")
            };

            println!("group '{}' message '{}' from '{}'" , q.chat.title().unwrap_or(""), q.text().unwrap_or(""), full_name);

            respond(())
        })
    );

    let filter_edited_message_handler = Update::filter_edited_message().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            println!("edited message");

            respond(())
        })
    );

    let filter_channel_post_handler = Update::filter_channel_post().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            //println!("channel post {} {}" , q.caption().unwrap_or(""), q.chat.title().unwrap_or(""));


            if q.chat.title().unwrap_or("") == "醋鸡导航总览" {
                
                if q.text().unwrap_or("") == "醋鸡" {
                    bot.unpin_all_chat_messages(q.chat.id).await.unwrap();
                    bot.delete_message(q.chat.id, q.id).await.unwrap();

                    let outer_good_link_buttons = vec![
                        InlineKeyboardButton{ 
                            text: String::from("上海修车指南总群"), 
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/shanghaisinan").unwrap())
                        },
                        InlineKeyboardButton{
                            text: String::from("上海修车师傅"),
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/fjjjnc").unwrap())
                        },
                        InlineKeyboardButton{
                            text: String::from("沪上天堂"),
                            kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/YYDSHsTtPPcB").unwrap())
                        }
                    ];

                    let mut outer_good_link_markup = InlineKeyboardMarkup::default();
                    
                    outer_good_link_buttons.iter().for_each(|x| {
                        let outer_good_link_markup_ref = &mut outer_good_link_markup;
                        outer_good_link_markup_ref.append_row(vec![x.clone()]);
                    });

                    
                    let new_message = bot.send_message(q.chat.id, "外部群 醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡")
                    .reply_markup(
                        outer_good_link_markup
                        // InlineKeyboardMarkup::default()
                        // .append_row(vec![
                        //     InlineKeyboardButton{ 
                        //         text: String::from("上海修车指南总群"), 
                        //         kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/shanghaisinan").unwrap())
                        //     }
                        // ])
                        // .append_row(vec![
                        //     InlineKeyboardButton{
                        //         text: String::from("上海修车师傅"),
                        //         kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/fjjjnc").unwrap())
                        //     }
                        // ])
                        // .append_row(vec![
                        //     InlineKeyboardButton{
                        //         text: String::from("沪上天堂"),
                        //         kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/YYDSHsTtPPcB").unwrap())
                        //     }
                        // ])
                    ).await.unwrap();
                    
                    bot.pin_chat_message(q.chat.id, new_message.id).await.unwrap();
                }
            }


            // if q.text().unwrap_or("") == "醋鸡" {

            //     bot.unpin_all_chat_messages(q.chat.id).await.unwrap();
            //     bot.delete_message(q.chat.id, q.id).await.unwrap();
                    
            //         let new_message = bot.send_message(q.chat.id, "醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡醋鸡")
            //         .reply_markup(
            //             InlineKeyboardMarkup::default()
            //             .append_row(vec![
            //                 InlineKeyboardButton{ 
            //                     text: String::from("浦东新区"), 
            //                     kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+W4Yki78fMccyM2Y1").unwrap())
            //                 }
            //             ])
            //             .append_row(vec![
            //                 InlineKeyboardButton{
            //                     text: String::from("碧之海 崂山路"),
            //                     kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
            //                 },
            //                 InlineKeyboardButton{
            //                     text: String::from("虹之间 沪南路"),
            //                     kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
            //                 }
            //             ])
            //             .append_row(vec![
            //                 InlineKeyboardButton{
            //                     text: String::from("心愿养生 五莲路"),
            //                     kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
            //                 }
            //             ])
            //         ).await.unwrap();
                    
            //         bot.pin_chat_message(q.chat.id, new_message.id).await.unwrap();
            // }

            respond(())
        })
    );

    let filter_edited_channel_post_handler = Update::filter_edited_channel_post().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            println!("edited channel post");

            respond(())
        })
    );

    let filter_inline_query_handler = Update::filter_inline_query().branch(dptree::endpoint(
            |bot: Bot, q: InlineQuery| async move {
                
                if q.query != "Nav" {
                    return respond(());
                }

                // First, create your actual response
                let google_search = InlineQueryResultArticle::new(
                    // Each item needs a unique ID, as well as the response container for the
                    // items. These can be whatever, as long as they don't
                    // conflict.
                    "01".to_string(),
                    // What the user will actually see
                    "Google Search",
                    // What message will be sent when clicked/tapped
                    InputMessageContent::Text(InputMessageContentText::new(format!(
                        "https://www.google.com/search?q={}",
                        q.query,
                    ))),
                );
                // While constructing them from the struct itself is possible, it is preferred
                // to use the builder pattern if you wish to add more
                // information to your result. Please refer to the documentation
                // for more detailed information about each field. https://docs.rs/teloxide/latest/teloxide/types/struct.InlineQueryResultArticle.html
                let ddg_search = InlineQueryResultArticle::new(
                    "02".to_string(),
                    "DuckDuckGo Search".to_string(),
                    InputMessageContent::Text(InputMessageContentText::new(format!(
                        "https://duckduckgo.com/?q={}",
                        q.query
                    ))),
                )
                .description("DuckDuckGo Search")
                .thumb_url("https://duckduckgo.com/assets/logo_header.v108.png".parse().unwrap())
                .url("https://duckduckgo.com/about".parse().unwrap()); // Note: This is the url that will open if they click the thumbnail
    

                let contact = InlineQueryResultContact::new(
                    "03".to_string(),
                    "1234567890".to_string(),
                    "John".to_string(),
                )
                .thumb_url("https://duckduckgo.com/assets/logo_header.v108.png".parse().unwrap());

                let results = vec![
                    InlineQueryResult::Article(google_search),
                    InlineQueryResult::Article(ddg_search),
                    InlineQueryResult::Contact(contact)
                ];

                // let maps = HashMap::new();

                // maps.insert("Shanghai", vec![
                //     InlineQueryResult::Article(google_search),
                //     InlineQueryResult::Article(ddg_search),
                //     InlineQueryResult::Contact(contact)
                // ]);

                // maps.insert("Shanghai-Xuhui", vec![
                //     InlineQueryResult::Article(google_search),
                //     InlineQueryResult::Article(ddg_search),
                //     InlineQueryResult::Contact(contact)
                // ]);
    
                // Send it off! One thing to note -- the ID we use here must be of the query
                // we're responding to.
                let response = bot.answer_inline_query(&q.id, results).send().await;
                if let Err(err) = response {
                    log::error!("Error in handler: {:?}", err);
                }
                respond(())
            },
        ));

    let filter_chosen_inline_result_handler = Update::filter_chosen_inline_result().branch(
        dptree::endpoint(|bot: Bot, q: ChosenInlineResult| async move {
            println!("chosen inline result");

            respond(())
        })
    );

    let filter_callback_query_handler = Update::filter_callback_query().branch(
        dptree::endpoint(|bot: Bot, q: CallbackQuery| async move {
            println!("callback query");

            respond(())
        })
    );

    let filter_shipping_query_handler = Update::filter_shipping_query().branch(
        dptree::endpoint(|bot: Bot, q: ShippingQuery| async move {
            println!("shipping query");

            respond(())
        })
    );

    let filter_pre_checkout_query_handler = Update::filter_pre_checkout_query().branch(
        dptree::endpoint(|bot: Bot, q: PreCheckoutQuery| async move {
            println!("pre checkout query");

            respond(())
        })
    );

    let filter_poll_handler = Update::filter_poll().branch(
        dptree::endpoint(|bot: Bot, q: Poll| async move {
            println!("poll");

            respond(())
        })
    );

    let filter_poll_answer_handler = Update::filter_poll_answer().branch(
        dptree::endpoint(|bot: Bot, q: PollAnswer| async move {
            println!("poll answer");

            respond(())
        })
    );

    let filter_my_chat_member_handler = Update::filter_my_chat_member().branch(
        dptree::endpoint(|bot: Bot, q: ChatMemberUpdated| async move {
            println!("my chat member");

            respond(())
        })
    );

    let filter_chat_member_handler = Update::filter_chat_member().branch(
        dptree::endpoint(|bot: Bot, q: ChatMemberUpdated| async move {
            println!("chat member");

            respond(())
        })
    );

    let filter_chat_join_request_handler = Update::filter_chat_join_request().branch(
        dptree::endpoint(|bot: Bot, q: ChatMemberUpdated| async move {
            println!("chat join request");

            respond(())
        })
    );


    let mut dispatcher = Dispatcher::builder(bot.clone(), dptree::entry()
    .branch(Update::filter_message().filter_command::<Command>().endpoint(answer))
    .branch(filter_message_handler) //
    .branch(filter_edited_message_handler) // 
    .branch(filter_channel_post_handler) //
    .branch(filter_edited_channel_post_handler) //
    .branch(filter_inline_query_handler) //
    .branch(filter_chosen_inline_result_handler) 
    .branch(filter_callback_query_handler) 
    .branch(filter_shipping_query_handler) 
    .branch(filter_pre_checkout_query_handler) 
    .branch(filter_poll_handler) 
    .branch(filter_poll_answer_handler) 
    .branch(filter_my_chat_member_handler) 
    .branch(filter_chat_member_handler) 
    .branch(filter_chat_join_request_handler))
    .enable_ctrlc_handler().build();

    dispatcher.dispatch().await;
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

async fn message_resp() -> ResponseResult<()> {
    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    println!("handle command {}", msg.chat.id);

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
                        kind: InlineKeyboardButtonKind::Pay(True)
                    }
                ])
                // .append_row(vec![
                //     InlineKeyboardButton{ 
                //         text: String::from("Club0"), 
                //         kind: InlineKeyboardButtonKind::SwitchInlineQuery(String::from("Club0"))
                //     },
                //     InlineKeyboardButton{ 
                //         text: String::from("Green Sea"), 
                //         kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/ljzty9999").unwrap())
                //     },
                //     InlineKeyboardButton{ 
                //         text: String::from("Forward"), 
                //         kind: InlineKeyboardButtonKind::SwitchInlineQuery(String::from("Club1"))
                //     },
                // ])
            ).await?
        }
    };

    Ok(())
}