use crate::structs::user::{UserList, FollowersResult};
use crate::database::tiktok::{user_exists, create_user};
use crate::database::connection::_get_connection;
use crate::http::http::make_request;
use std::time::Instant;
use std::collections::HashMap;
use anyhow::{Error, Result, anyhow};


pub async fn get_follow_scene(secuid: &str, max_cursor: isize, min_cursor: isize, scene: i8) -> Result<UserList, Error> {
    // Format TikTok GET-Endpoint
    // 67 == Followers
    // 66??? == Following
    let base_url: String = "https://www.tiktok.com/api/user/list/?count=200&minCursor=VAR_MIN_CURSOR&maxCursor=VAR_MAX_CURSOR&scene=VAR_SCENE&secUid=VAR_SUID".to_string();
    let start_url: String = base_url.replace("VAR_SUID", &secuid)
    .replace("VAR_MAX_CURSOR", &max_cursor.to_string())
    .replace("VAR_MIN_CURSOR", &min_cursor.to_string())
    .replace("VAR_SCENE", &scene.to_string());

    let client = reqwest::Client::new();
    let resp_text = make_request(&start_url, client).await;
    match resp_text {
        Ok(val) => {
            // !DEBUG 
            println!("Ma-{}/Mi-{} | C-{}", max_cursor, min_cursor, start_url);
            let response_json: UserList = serde_json::from_str(&val)?;
            Ok(response_json)
        },
        Err(_e) => {
            println!("{}", _e);
            return Err(anyhow!("Bruh"));
        }
    }
}


pub async fn iterate_follows(input_suid: &str, scene: i8) -> Result<HashMap<String, FollowersResult>, Error> {
    let mut min_cursor: isize = 0;
    let mut max_cursor: isize = 0;
    let mut result_packet: HashMap<String, FollowersResult> = HashMap::new();

    loop {
        let start_time = Instant::now();

        // Parse JSON and check it
        let response_json: Result<UserList, Error> = get_follow_scene(&input_suid, max_cursor, min_cursor, scene).await;
        match response_json {
            Ok(json) => {
            // Iterate through  users in scenery
            for user in json.users {
                // Initialize result meta
                let mut user_follower_result = FollowersResult {
                    is_private: false,
                    is_secret: false,
                    already_checked: true
                };
                // Feedback
                println!("Nickname: {} | SUID: {} | Private: {}  | Secret: {}",
                    user.info.nickname,
                    &user.info.sec_uid,
                    user.info.is_private,
                    user.info.is_secret
                );

                // Insert user into database
                match user_exists(&user.info.sec_uid) {
                    Ok(check) => {
                        if !check {
                            // User does not seem to exist
                            let mut conn = _get_connection();
                            let trans = conn.transaction();
                            match create_user(&user, trans) {
                                Ok(()) => {
                                    println!("User has been added to the database. ({})", &user.info.nickname);
                                },
                                Err(e) => {
                                    println!("Error adding user to the database => {}", e);
                                }
                            }
                        } 
                    },
                    Err(e) => {
                        println!("Error whilst checking user's existance: {}", e)
                    }
                }


                // Housekeeping
                if user.info.is_secret {
                    user_follower_result.is_secret = true;
                    result_packet.insert(user.info.sec_uid.to_string(), user_follower_result);
                    continue;
                }
                if user.info.is_private {
                    user_follower_result.is_private = true;
                    result_packet.insert(user.info.sec_uid.to_string(), user_follower_result);
                    continue;
                }
                result_packet.insert(user.info.sec_uid.to_string(), user_follower_result);

            }
            println!("HasMore: {} | MiC: {} | MaC: {}",json.has_more, json.min_cursor, json.max_cursor);
            if !json.has_more  {
                println!("\nIteration cycle has been completed => No more pages for account scene\n");
                break Ok(result_packet);
            }
            min_cursor = json.min_cursor;
            max_cursor = json.max_cursor;
            },
            Err(e) => {
                println!("Errorf: {}", e);
                break Err(anyhow!("yeee"));
            }
        }
        let elapsed_time = start_time.elapsed().as_millis();
        println!("Time elapsed: {}", elapsed_time)
    }
}
