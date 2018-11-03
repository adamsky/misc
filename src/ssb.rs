extern crate rand;

use serde_json;
use serde_derive;
use serde;

use std::process::Command;

use serde_json::{Value, Deserializer, from_value};
use std::any::Any;
use std::collections::HashMap;

use std::process;

use sbot;

pub struct ChatInstance {
    pub network_name: String,
    pub msg_type: String,
    pub sbot_server: Option<process::Child>,
    pub user_names: HashMap<String, String>,
    pub user_name_colors: HashMap<String, (f32,f32,f32,f32)>,
    pub chat_messages: Vec<ChatMessage>,
    pub all_about_messages: Vec<Message>,
    pub unique_users: Vec<String>,
    pub current_user_handle: String,
}

impl ChatInstance {
    pub fn new(msg_type: String, network_name: String) -> ChatInstance {
        let sbot_server = sbot::new_sbot_server(&network_name).expect("failed starting sbot server");
        println!("started sbot server");

        let mut chat_messages = get_chat_messages_of_type(&msg_type, 100, &network_name);
        println!("success: get_chat_messages_of_type() = {}", chat_messages.len());
        let all_about_messages = get_messages_of_type("about", &network_name, true);
        println!("success: get_messages_of_type() = {}", all_about_messages.len());
        if all_about_messages.len() < 1 {
            println!("watch out: no about messages found!");
        }
        let unique_users = get_unique_users(chat_messages.clone());
        println!("success: get_unique_users() = {}", unique_users.len());
        let user_names = get_user_names(
            unique_users.clone(), all_about_messages.clone(), &network_name);
        println!("success: get_user_names()");

        println!("found matching usernames: {}", user_names.len());

        // update messages to contain user handles
        for mut chat_msg in &mut chat_messages {
            if user_names.contains_key(&chat_msg.author) {
                chat_msg.author_handle = user_names[&chat_msg.author].clone();
            }
        }

        println!("number of users: {}", unique_users.len());
        println!("number of chat_messages: {}", chat_messages.len());
//        let unique_users: Vec<String> = Vec::new();
//        let all_about_messages: Vec<Message> = Vec::new();
//        let user_names: HashMap<String, String> = HashMap::new();

        let mut user_name_colors = get_user_name_colors(&unique_users);


        let current_user_handle = get_user_handle(whoami(&network_name), &network_name);

        ChatInstance {
            network_name,
            msg_type,
            sbot_server: Some(sbot_server),
            user_names,
            user_name_colors,
            chat_messages,
            all_about_messages,
            unique_users,
            current_user_handle,
        }
    }
    pub fn format_messages_to_string(&self) -> String {
        let mut out_string: String = String::new();
        for msg in &self.chat_messages {

            out_string.push_str(&msg.author_handle);
            out_string.push_str(": ");
            out_string.push_str(&msg.text);
            out_string.push_str("\n");
        }
        out_string
    }
    pub fn kill(&mut self) {
        match &mut self.sbot_server {
            None => (),
            Some(sbot) => {
                sbot.kill();
            },
        };
    }
    pub fn publish_message(&mut self, text: String) {
        Command::new("sbot")
            .env("ssb_appname", &self.network_name)
            .arg("publish")
            .arg("--type")
            .arg(&self.msg_type)
            .arg("--text")
            .arg(text)
            .output()
            .expect("failed to execute process");

        self.refresh();
    }
    // reload messages?
    pub fn refresh(&mut self) {
        self.chat_messages = get_chat_messages_of_type(&self.msg_type, 100, &self.network_name);
        // update messages to contain user handles
        for mut chat_msg in &mut self.chat_messages {
            if self.user_names.contains_key(&chat_msg.author) {
                chat_msg.author_handle = self.user_names[&chat_msg.author].clone();
            }
        }
    }

}



#[derive(Deserialize, Clone)]
pub struct Message {
    key: String,
    timestamp: Value,
    value: MessageValues,
}


#[derive(Deserialize, Clone)]
pub struct MessageValues {
    author: String,
    timestamp: Value,
    content: MessageContent,
}
#[derive(Deserialize, Clone)]
pub struct MessageContent {
    #[serde(rename = "type")]
    type_: String,
    text: Option<String>,
    about: Option<String>,
    name: Option<String>,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub type_: String,
    pub author: String,
    pub author_handle: String,
    pub text: String,
}

pub fn get_chat_messages_of_type(msg_type: &str, limit: i32, network_name: &String) -> Vec<ChatMessage> {
    let mut all_msgs = get_messages_of_type(msg_type, network_name, false);
    println!("success: get_messages_of_type()");
    let mut chat_msgs: Vec<ChatMessage> = Vec::new();

    let mut limited_msgs: Vec<Message> = Vec::new();
    for n in (0..limit).rev() {
        if all_msgs.len() <= n as usize {
            continue;
        }
        limited_msgs.push(all_msgs[all_msgs.len() - n as usize - 1].clone());
    }
    for msg in limited_msgs {
        chat_msgs.push(chat_message_from_message(msg));
    }
    chat_msgs
}

fn get_messages_of_type(msg_type: &str, network_name: &String, reverse: bool) -> Vec<Message> {
    let cmd_output = match reverse {
        false => Command::new("sbot")
            .env("ssb_appname", network_name)
            .arg("messagesByType")
            .arg(msg_type)
            .output()
            .expect("failed to execute process"),
        true => Command::new("sbot")
            .env("ssb_appname", network_name)
            .arg("messagesByType")
            .arg(msg_type)
            .arg("--reverse")
            .output()
            .expect("failed to execute process"),
    };

//    println!("{:?}", &cmd_output.stdout);
    let mut out_vec: Vec<Message> = Vec::new();
    let stream = Deserializer::from_slice(&cmd_output.stdout).into_iter::<Value>();
//    if stream.count().clone() == 0 {
//        return Vec::new();
//    }
    for value in stream {
//        println!("{}", msg_type);
//        println!("{:?}", &value);
        let v = match value {
            Ok(v) => v,
            Err(e) => {
                continue;
            },
        };
        // skip malformed fields
        // for some reason some content.text fields contain a map instead of a string
        if msg_type == "scat_message" && !v.clone()["value"]["content"]["text"].is_string() {
            continue;
        }
        //
        if msg_type == "about" && !v.clone()["value"]["content"]["name"].is_string() {
            continue;
        }
        out_vec.push(from_value(v).expect("failed from_value"));
    }
    out_vec
}

fn chat_message_from_message(msg: Message) -> ChatMessage {
    ChatMessage {
        type_: msg.value.content.type_,
        author: msg.value.author.clone(),
        author_handle: msg.value.author.clone(),
        text: msg.value.content.text.unwrap(),
    }
}

//#[derive(Deserialize)]
//struct PartialMessage {
//    value: Value,
//}

pub fn get_user_handle(user_pubkey: String, network_name: &String) -> String {
    // first get user's messages
    let cmd_output =
        Command::new("sbot")
            .env("ssb_appname", network_name)
            .arg("createUserStream")
            .arg("--id")
            .arg(user_pubkey.clone())
            .arg("--reverse")
            .output()
            .expect("failed to execute process");

    let stream = Deserializer::from_slice(&cmd_output.stdout).into_iter::<Value>();
    let mut newest_username: String = String::from("username_not_found");
    for value in stream {
        let v = value.unwrap();
//        println!("{:?}", v);
        if v["value"]["content"]["about"].is_string() && v["value"]["content"]["about"].as_str().unwrap() == user_pubkey {
            if v["value"]["content"]["type"].is_string() {
                if v["value"]["content"]["type"].as_str().expect("no type?") == "about" {
                    if v["value"]["content"]["name"].is_string() {
//                    newest_username = v["value"]["content"]["name"].as_str().unwrap().to_string();
                        return v["value"]["content"]["name"].as_str().unwrap().to_string();
                    }
                }
            }
        }
    }
    newest_username
}

fn get_unique_users(chat_msgs: Vec<ChatMessage>) -> Vec<String> {
    let mut out_vec: Vec<String> = Vec::new();
    for msg in chat_msgs {
        if !out_vec.contains(&msg.author) {
            out_vec.push(msg.author)
        }
    }
    out_vec
}

fn get_user_names(unique_users: Vec<String>, about_msgs: Vec<Message>, network_name: &String) -> HashMap<String, String> {
    let mut out_map = HashMap::new();
    'outer: for user in unique_users {

        for about_msg in about_msgs.clone().iter().rev() {
//            println!("{}", about_msg.value.author);
            if user == about_msg.value.author {
                if &user == about_msg.value.content.about.as_ref().unwrap() {
                    let name = about_msg.value.content.name.as_ref().unwrap();
//                    println!("Success! {} + {} ", user, name.clone());
                    out_map.insert(user, name.to_owned());
                    continue 'outer;
                }
            }
        }
//        out_map.insert(user.clone(), get_user_handle(user, &network_name));
    }
    out_map
}

fn get_user_name_colors(users: &Vec<String>) -> HashMap<String, (f32,f32,f32,f32)> {
    let mut out_map: HashMap<String, (f32,f32,f32,f32)> = HashMap::new();

    for user in users {
//        let mut col_mod: f32 = 0.0;
//        for ch in user.chars() {
//            col_mod += char_to_color_float(ch);
//        }
//        println!("{}", col_mod);
//        col_mod = col_mod / 100.0;
//        out_map.insert(user.to_string(), (col_mod*1.2, col_mod*1.3, col_mod*1.4, 0.9 ));

        use self::rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        out_map.insert(user.to_string(),
                       (rng.gen_range(0.4, 1.0), rng.gen_range(0.4, 1.0), rng.gen_range(0.4, 1.0), 0.9));
    };

    out_map

}
fn char_to_color_float(ch: char) -> f32 {
    match ch.to_string().as_str() {
        "a" | "A" => 1.0 / 26.0,
        "b" | "B" => 1.0 / 26.0 * 2.0,
        "c" | "C" => 1.0 / 26.0 * 3.0,
        "d" | "D" => 1.0 / 26.0 * 4.0,
        "e" | "E" => 1.0 / 26.0 * 5.0,
        "f" | "F" => 1.0 / 26.0 * 6.0,
        "g" | "G" => 1.0 / 26.0 * 7.0,
        "h" | "H" => 1.0 / 26.0 * 8.0,
        "i" | "I" => 1.0 / 26.0 * 9.0,
        "j" | "J" => 1.0 / 26.0 * 10.0,
        "k" | "K" => 1.0 / 26.0 * 11.0,
        "l" | "L" => 1.0 / 26.0 * 12.0,
        "m" | "M" => 1.0 / 26.0 * 13.0,
        "n" | "N" => 1.0 / 26.0 * 14.0,
        "o" | "O" => 1.0 / 26.0 * 15.0,
        "p" | "P" => 1.0 / 26.0 * 16.0,
        "q" | "Q" => 1.0 / 26.0 * 17.0,
        "r" | "R" => 1.0 / 26.0 * 18.0,
        "s" | "S" => 1.0 / 26.0 * 19.0,
        "t" | "T" => 1.0 / 26.0 * 20.0,
        "u" | "U" => 1.0 / 26.0 * 21.0,
        "v" | "V" => 1.0 / 26.0 * 22.0,
        "w" | "W" => 1.0 / 26.0 * 23.0,
        "x" | "X" => 1.0 / 26.0 * 24.0,
        "y" | "Y" => 1.0 / 26.0 * 25.0,
        "z" | "Z" => 1.0 / 26.0 * 26.0,
        _ => 1.0 / 26.0 * 10.0,
    }
}

#[derive(Deserialize)]
struct Whoami {
    id: String,
}

pub fn whoami(network_name: &String) -> String {
    let cmd_output =
        Command::new("sbot")
            .env("ssb_appname", network_name)
            .arg("whoami")
            .output()
            .expect("failed to execute process");

    let whoami: Whoami = serde_json::from_slice(&cmd_output.stdout).expect("failed whoami");
    whoami.id
}
