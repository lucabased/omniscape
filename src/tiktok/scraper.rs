use std::collections::HashMap;
use crate::tiktok::follow::iterate_follows;
use crate::structs::user::FollowersResult;


pub async fn start_scraper(start_suid: String) {
    // TODO: Add some dynamics e.g. following etc.
    let scene: i8 = 67;
    let initial_list: HashMap<String, FollowersResult> = iterate_follows(&start_suid, scene.clone()).await.unwrap();
    let mut result_store: HashMap<String, FollowersResult> = HashMap::new();
    // Recursively iterate
    loop {
        for (list_key, list_value) in initial_list.iter() {
            let list_children = iterate_follows(&list_key, scene).await;
            match list_children {
                Ok(val) => {
                    if !list_value.is_private || !list_value.is_secret {
                        for (child_key, _child_value) in val {
                            result_store.insert(child_key, _child_value);
                            println!("ResultStore Length: {}", result_store.len())
                        }
                    }
                    
                },
                Err(e) => {
                    println!("We fucked up bad => {}", e)
                }
            }
            
        }
    }
    // TODO: Add an infinite iteration of result_store to scrape entire tiktok
    
}
