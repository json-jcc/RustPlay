use teloxide::{
    prelude::*, 
    utils::command::BotCommands, 
    types::*,
    requests::{Requester, ResponseResult},
};
use tokio::join;

use std::{env, collections::HashMap, fs, path::Path};
use serde_json;
use serde::{Serialize, Deserialize};
use timer::Timer;


const TELOXIDE_TOKEN: &str = "6402266107:AAFSAZN2r1fxJRrvvCw4wexI1b9wKJwOYgM";
const PROVIDER_TOKEN: &str = "5322214758:TEST:afdd97df-8d23-47a4-87d2-06fb2f285595";
const CHANNEL_TOTAL_CHAT_ID: ChatId = ChatId(-1002057929576); // https://github.com/GabrielRF/telegram-id#app-channel-id


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum TuiYouLevel {
    _2t,    
    _4t,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TuiYouInfo {
    district: String,
    name: String,
    link: String,
}

fn send_admission_ticket_invoice<Ch>(bot: Bot, chat_id: Ch) -> teloxide::requests::JsonRequest<teloxide::payloads::SendInvoice> 
where Ch: Into<Recipient> {
    bot.send_invoice(chat_id, "入场券", "一次购买终身入场", "aaaa", 
                PROVIDER_TOKEN, 
                "CNY", vec![
                    LabeledPrice{label: String::from("券面"), amount: 5000}, // 50.00
                    LabeledPrice{label: String::from("增值税"), amount: 1000} // 10.00
                    ])
                .reply_markup(InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton{ 
                            text: String::from("购买入场券"), 
                            kind: InlineKeyboardButtonKind::Pay(True)
                        }
                    ]
                ]))
}

fn send_outer_links_message<Ch>(bot: Bot, chat_id: Ch) -> teloxide::requests::MultipartRequest<teloxide::payloads::SendPhoto>
where Ch: Into<Recipient> {
    bot.send_photo(chat_id, InputFile::file(std::path::Path::new("database/pic/FuckYou1.jpg")))
    .reply_markup(InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton{ 
                text: String::from("上海修车指南总群"), 
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/shanghaisinan").unwrap())
            }
        ],
        vec![
            InlineKeyboardButton{
                text: String::from("上海修车师傅"),
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/fjjjnc").unwrap())
            }
        ],
        vec![
            InlineKeyboardButton{
                text: String::from("沪上天堂"),
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/YYDSHsTtPPcB").unwrap())
            }
        ]
    ]))
}

fn send_ty_links_message(bot: Bot, chat_id: ChatId) -> Vec<teloxide::requests::MultipartRequest<teloxide::payloads::SendPhoto>>
{
    let dir = Path::new("database/ty/");
    let paths = fs::read_dir(dir).unwrap();
    let mut infos = vec![];
    
    paths.into_iter().for_each(|path| {
        let path = path.unwrap();
        let json = fs::read_to_string(path.path()).unwrap();

        let mut sub_infos = serde_json::from_str::<Vec<TuiYouInfo>>(json.as_str()).unwrap_or_default();
        infos.append(&mut sub_infos);
    });
    
    let mut mapped_infos = HashMap::new();
    
    infos.iter().for_each(|info| {
        if !mapped_infos.contains_key(&info.district) {
            mapped_infos.insert(info.district.clone(), vec![info.clone()]);
        } else { 
            mapped_infos.get_mut(&info.district).unwrap().push(info.clone());
        }
    });

    mapped_infos.iter().map(|(district, infos)| {
        let mut rows = <Vec<Vec<InlineKeyboardButton>>>::new();

        let mut row = Vec::new();
        let mut i = 0;

        infos.iter().for_each(|info| {
            if i == 0 {
                row = Vec::new();
                row.push(InlineKeyboardButton{
                    text: info.name.clone(),
                    kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse(&info.link).unwrap())
                });
                i = i + 1;
            } else if i == 1 {
                row.push(InlineKeyboardButton{
                    text: info.name.clone(),
                    kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse(&info.link).unwrap())
                });
                i = 0;
                rows.push(row.clone());
            }
        });

        rows.push(row.clone());

        bot.send_photo(chat_id, InputFile::file(std::path::Path::new("database/pic/FuckYou2.jpg")))
            .caption(district.as_str())
            .reply_markup(InlineKeyboardMarkup::new(rows))
    }).collect()
}

fn get_nav_query_result(bot: Bot, q: InlineQuery) -> teloxide::requests::JsonRequest<teloxide::payloads::AnswerInlineQuery> {
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

    bot.answer_inline_query(&q.id, results)
}

fn send_sub_channels(bot: Bot, chat_id: ChatId) -> teloxide::requests::MultipartRequest<teloxide::payloads::SendPhoto> {
    bot.send_photo(chat_id, InputFile::file(std::path::Path::new("database/pic/FuckYou1.jpg")))
    .reply_markup(InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton{ 
                text: String::from("醋鸡导航（上海）"), 
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+L59NF2B-wdo5MmRl").unwrap())
            }
        ],
        vec![
            InlineKeyboardButton{
                text: String::from("醋鸡导航（深圳）"),
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+TarGf934aL4yMzM1").unwrap())
            }
        ],
        vec![
            InlineKeyboardButton{
                text: String::from("醋鸡导航（广州）"),
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+rHe_VspU3kwxMTM1").unwrap())
            }
        ],
        vec![
            InlineKeyboardButton{
                text: String::from("醋鸡导航（南京）"),
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+j4x9suReBeI0YTVl").unwrap())
            }
        ],
        vec![
            InlineKeyboardButton{
                text: String::from("醋鸡导航（合肥）"),
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+XflyABk3gR82ZWQ1").unwrap())
            }
        ]
    ]))
}

fn send_return_to_top_channel(bot: Bot, chat_id: ChatId) -> teloxide::requests::MultipartRequest<teloxide::payloads::SendPhoto> {
    bot.send_photo(chat_id, InputFile::file(std::path::Path::new("database/pic/FuckYou1.jpg")))
    .reply_markup(InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton{ 
                text: String::from("醋鸡导航总览"), 
                kind: InlineKeyboardButtonKind::Url(reqwest::Url::parse("https://t.me/+gfOT7qEet-RkNWZl").unwrap())
            }
        ]
    ]))
}

#[tokio::main]
async fn main() {
    env::set_var("TELOXIDE_TOKEN", TELOXIDE_TOKEN);

    pretty_env_logger::init();
    let bot = Bot::from_env();

    bot.get_updates().await.unwrap().iter().for_each(|update|{
    });

    bot.send_message(CHANNEL_TOTAL_CHAT_ID, "醋鸡火车头已上线。").await.unwrap();

    let filter_message_handler = Update::filter_message().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            
            println!("message {}", q.text().unwrap_or(""));

            if q.text().unwrap_or("") == "醋鸡" {
                bot.delete_message(q.chat.id, q.id).await.unwrap();
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
            println!("chat id {}", q.chat.id);
            if q.text().unwrap_or("") == "醋鸡" {
                bot.delete_message(q.chat.id, q.id).await.unwrap();

                if q.chat.title().unwrap_or("") == "醋鸡导航总览" {
                    send_sub_channels(bot.clone(), q.chat.id).await.unwrap();
                }
    
                if q.chat.title().unwrap_or("") == "醋鸡导航（上海）" {
                    send_outer_links_message(bot.clone(), q.chat.id).await.unwrap();
                    for rq in send_ty_links_message(bot.clone(), q.chat.id.clone()) {
                        rq.await.unwrap();
                    }
                    send_return_to_top_channel(bot.clone(), q.chat.id).await.unwrap();
                }
            }

            respond(())
        })
    );

    let filter_edited_channel_post_handler = Update::filter_edited_channel_post().branch(
        dptree::endpoint(|bot: Bot, q: Message| async move {
            println!("edited channel post");
            respond(())
        })
    );

    // CuJiNavBot
    // 对机器人使用查询关键词
    let filter_inline_query_handler = Update::filter_inline_query().branch(
        dptree::endpoint(
            |bot: Bot, q: InlineQuery| async move {
                if q.query == "Nav" {
                    // Send it off! One thing to note -- the ID we use here must be of the query
                    // we're responding to.
                    //let response = bot.answer_inline_query(&q.id, results).send().await;
                    let response = get_nav_query_result(bot, q).await;
                    if let Err(err) = response {
                        log::error!("Error in handler: {:?}", err);
                    }
                }
                respond(())
            })
    );

    // 当选中任意一个查询结果
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

    let mut dispatcher = Dispatcher::builder(
        bot.clone(), 
        dptree::entry()
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
        .branch(filter_chat_join_request_handler)
    )
    .enable_ctrlc_handler().build();

    // let timer = Timer::new();

    // let bot_x = bot.clone();
    // timer.schedule_repeating(chrono::Duration::milliseconds(5000), move || {
    //     {
    //        sceduled_events(bot_x.clone()).await;
    //     }
    // });

    dispatcher.dispatch().await;
    //let a = join!(dispatcher.dispatch(), sceduled_events(bot.clone()));

    bot.send_message(CHANNEL_TOTAL_CHAT_ID, "醋鸡火车头已下线。").await.unwrap();
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return a test button.")]
    NavApp,
}

async fn sceduled_events(bot: Bot) {
    bot.send_message(CHANNEL_TOTAL_CHAT_ID, "醋鸡火车头 PPPP。").await.unwrap();
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    println!("handle command {}", msg.chat.id);

    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::NavApp => {
            bot.send_message(msg.chat.id, "This is a fast navigation.")
            .reply_markup(
                InlineKeyboardMarkup::default()
                .append_row(vec![
                    InlineKeyboardButton{ 
                        text: String::from("Baidu"), 
                        kind: InlineKeyboardButtonKind::WebApp(WebAppInfo { url: reqwest::Url::parse("https://www.baidu.com").unwrap() })
                    },
                    InlineKeyboardButton{
                        text: String::from("Google"), 
                        kind: InlineKeyboardButtonKind::WebApp(WebAppInfo { url: reqwest::Url::parse("https://www.google.com").unwrap() })
                    }
                ])
            ).await?
        }
    };

    Ok(())
}