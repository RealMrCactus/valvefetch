use clap::Parser;
use indicatif::ProgressBar;
use serde_json::Value;
use std::{fs::read_to_string, path::PathBuf, process::Stdio};
use tokio::{io::AsyncBufReadExt, process::Command};

#[derive(Parser, Debug)]
#[clap(about = "A SteamCMD wrapper for managing Steam Workshop content")]
struct Args {
    /// Steam username for authentication
    #[clap(long, short, default_value = "anonymous")]
    login: Option<String>,
    /// Workshop item ID or URL to download
    #[clap(long, short)]
    download: Option<i64>,
    /// Custom installation path
    #[clap(long, value_parser)]
    path: Option<PathBuf>,
    /// Save current path as default
    #[clap(long, short)]
    save: bool,
    /// Path to batch file containing workshop IDs
    #[clap(long, short, value_parser)]
    batch: Option<String>,
}

async fn find_path() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd")
            .arg("/C")
            .arg("where steamcmd")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdout) = cmd.stdout.take() {
            let reader = tokio::io::BufReader::new(stdout);
            let mut lines = reader.lines();
            
            // Get the first line (first path found)
            if let Some(line) = lines.next_line().await.expect("Some fucking shit.") {
                return Ok(line);
            }
        }
        Err("SteamCMD not found".into())
    } else if cfg!(target_os = "linux") {
        let mut cmd = Command::new("sh")
            .arg("-c")
            .arg("\"which steamcmd\"")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdout) = cmd.stdout.take() {
            let reader = tokio::io::BufReader::new(stdout);
            let mut lines = reader.lines();
            // Get the first line (first path found)
            if let Some(line) = lines.next_line().await.expect("Some fucking shit.") {
                return Ok(line);
            }
        }
        Err("SteamCMD not found".into())
    } else {
        Err("Not implemented for this platform".into())
    }
}

#[derive(Debug, Clone)]
struct WorkshopItem {
    url: String,
    item_id: Option<i64>,
    game_id: Option<i64>,
}

async fn get_workshop_item(shop: &WorkshopItem) -> Option<i64> {
    println!("{}", shop.item_id.expect("Item ID received is none @ 41"));

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("itemcount=1&publishedfileids[0]={}", shop.item_id.expect("Item ID received is none @ 41")))
        .send()
        .await
        .ok()?;
    let json: Value = response.json().await.ok()?;
    
    println!("{}", json["response"]["publishedfiledetails"][0]["consumer_app_id"]);
    json["response"]["publishedfiledetails"][0]["consumer_app_id"]
        .as_i64()
}

async fn command(download: bool, shop: WorkshopItem, login: String) {
    if download {
        if ! login.is_empty() {
            Command::new("steamcmd")
                .arg(format!("+login {} +workshop_download_item {:?} {:?} +quit", login, shop.game_id, shop.item_id));
        } else {
            panic!("Failed to run command.\n    download: {:?}\n    shop: {:?}\n    login: {:?}", download, shop, login)
        }
    }
}

async fn batch(file: String, login: String) {
    println!("{}", file);
    
    let lines: u64 = contents.lines().count() as u64;
    let bar = ProgressBar::new(lines);
    let contents: String = read_to_string(file).expect("Could not read batch file");
    let mut shop = WorkshopItem {
        url: String::new(),
        item_id: 0.into(),
        game_id: 0.into(),
    };

    bar.set_message("Downloading addons");
    
    for line in contents.lines() {
        bar.inc(1);
        shop.item_id = Some(line.parse::<i64>().expect(&format!("Error parsing item ID '{}'", line)));
        command(true, shop.clone(), login.clone()).await;
    }
    bar.finish();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let path_result = find_path().await;
    let steamcmd = format!("{:?}", path_result);
    println!("SteamCMD Path: {:?}", steamcmd);
    
    if args.download.is_some() {
        let login_string: String;
        let mut workshop_item = WorkshopItem {
            url: String::new(),
            item_id: 0.into(),
            game_id: 0.into(),
        };
        
        workshop_item.item_id = args.download;
        workshop_item.game_id = get_workshop_item(&workshop_item).await;
        
        println!("Game ID: {}", workshop_item.game_id.expect("Failed to get Game ID"));

        if ! args.login.is_none() {
            login_string = args.login.as_ref().unwrap().to_string();
        } else {
            login_string = "".to_string();
        }

        command(true, workshop_item, login_string).await
    } else if args.batch.is_some() {        
        let login_string: String;
        
        if ! args.login.is_none() {
            login_string = args.login.as_ref().unwrap().to_string();
        } else {
            login_string = "".to_string();
        }

        batch(format!("{}", args.batch.unwrap()), login_string).await
    }
}
