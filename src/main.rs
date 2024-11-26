//! This example illustrates the way to send and receive arbitrary JSON.
//!
//! This is useful for some ad-hoc experiments and situations when you don't
//! really care about the structure of the JSON and just need to display it or
//! process it at runtime.

//1. 找到系列数据
//2. 分别有哪几季？
//3. 请求每一季的数据
//4. 解析季数据找到集数据，告诉我集数据在哪里就可以



// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
use reqwest::header::{HeaderValue, CONTENT_TYPE};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let mut num = String::new();
    // io::stdin().read_line(&mut num).expect("Error");
    let api_key_query = [
        ("api_key", "813659d1715d4e488fe5d6210e455234"),
        ("language", "zh-CN")
    ];
    const SERIES_ID: usize = 34742;
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("localhost:7890")?)
        .build()
        .unwrap();
        
    let echo_json = client
        .get(format!("https://api.themoviedb.org/3/tv/{SERIES_ID}"))
        .query(&api_key_query)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json") )
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .bytes()
        .await
        .unwrap();
    //println!("{body:#?}");
    let json = serde_json::from_slice::<serde_json::Value>(&echo_json).unwrap();
    let available_seasons = json["seasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|season| season["season_number"].as_u64().unwrap())
        .collect::<Vec<_>>();
    for season_number in available_seasons {
        println!("第{season_number}季");
        let res_json: serde_json::Value = client
            .get(format!("https://api.themoviedb.org/3/tv/{SERIES_ID}/season/{season_number}"))
            .query(&api_key_query)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        res_json["episodes"]
            .as_array()
            .unwrap()
            .iter()
            .for_each(|episode|{                
                let epnumber = episode["episode_number"].as_u64().unwrap();
                let epname = episode["name"].as_str().unwrap();
                println!("{epnumber}:{}",epname)
            })       
        }


    
    //let seasons_json = &body_json["seasons"][0];
    // https://api.themoviedb.org/3/tv/34742/season/0/episode/1?language=en-US
    Ok(())
    
}