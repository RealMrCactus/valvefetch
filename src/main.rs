use clap::Parser;
use indicatif::ProgressBar;
use std::{fs::read_to_string, path::PathBuf, process::Stdio};
use scraper::{Html, Selector};
use tokio::{io::AsyncBufReadExt, process::Command};

#[derive(Parser, Debug)]
#[clap(about = "A SteamCMD wrapper for managing Steam Workshop content")]
struct Args {
    /// Steam username for authentication
    #[clap(long)]
    login: Option<String>,
    /// Workshop item ID or URL to download
    #[clap(long)]
    download: Option<String>,
    /// Target game identifier (optional if URL is provided)
    #[clap(long)]
    game: Option<String>,
    /// Custom installation path
    #[clap(long, value_parser)]
    path: Option<PathBuf>,
    /// Save current path as default
    #[clap(long)]
    save: bool,
    /// Path to batch file containing workshop IDs
    #[clap(long, value_parser)]
    batch: Option<PathBuf>,
    /// Reduce output verbosity
    #[clap(long)]
    quiet: bool,
}

#[derive(Debug, Clone)]
struct WorkshopItem {
    url: String,
    item_id: i64,
    game_id: Option<i64>,
}

async fn get_workshop_item(shop: &WorkshopItem) -> Option<i64> {
    let request_url = if shop.url.is_empty() {
        format!("https://steamcommunity.com/sharedfiles/filedetails/?id={}", shop.item_id)
    } else {
        shop.url.clone()
    };

    let response = reqwest::get(&request_url).await.ok()?;
    let html_content = response.text().await.ok()?;
    let document = Html::parse_document(&html_content);
    
    let selector = Selector::parse("a.btnv6_blue_hoverfade").ok()?;
    
    document.select(&selector)
        .find_map(|element| {
            element.value()
                .attr("data-appid")
                .and_then(|app_id| app_id.parse::<i64>().ok())
        })
}

async fn command(download: bool, shop: WorkshopItem, login: String) {
    if download {
        if ! login.is_empty() {
            Command::new("steamcmd")
                .arg(format!("+login {} +workshop_download_item {:?} {:?}", login, shop.game_id, shop.item_id))
                .output().await.unwrap();
        } else if login.is_empty() {
            Command::new("steamcmd")
                .arg(format!("+login {} +workshop_download_item {:?} {:?}", "anonymous", shop.game_id, shop.item_id))
                .output().await.unwrap();
        } else {
            panic!("Somethings not right....")
        }
    }
}

async fn find_path() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd")
            .arg("/C")
            .arg("where steamcmd")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdout) = cmd.stdout.take() {
            let mut reader = tokio::io::BufReader::new(stdout);
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
            .arg("\"where steamcmd\"")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdout) = cmd.stdout.take() {
            let mut reader = tokio::io::BufReader::new(stdout);
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

async fn batch(file: String, shop: WorkshopItem, login: String) {
    let contents: String = read_to_string(file).expect("!! ERROR READING BATCH FILE TO STRING !!").trim().to_string();

    let mut lines: u64 = 0;
    
    for line in contents.lines() {
        lines += 1;
    } 

    let bar = ProgressBar::new(lines);

    for line in contents.lines() {
        bar.inc(1);
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
        let mut workshop_item = WorkshopItem {
            url: String::new(),
            item_id: 3362207896,
            game_id: None,
        };
        
        if let Some(game_id) = get_workshop_item(&workshop_item).await {
            println!("Game ID: {}", game_id);
            workshop_item.game_id = Some(game_id);
        } else {
            println!("Failed to retrieve game ID.");
        }
        
        let login_string = args.login.as_ref().unwrap().to_string();

        command(true, workshop_item, login_string).await
    } else if args.batch.is_some() {
        let mut workshop_item = WorkshopItem {
            url: String::new(),
            item_id: 3362207896,
            game_id: None,
        };
        
        if let Some(game_id) = get_workshop_item(&workshop_item).await {
            println!("Game ID: {}", game_id);
            workshop_item.game_id = Some(game_id);
        } else {
            println!("Failed to retrieve game ID.");
        }
        
        let login_string = args.login.as_ref().unwrap().to_string();

        batch(format!("{:?}", args.batch), workshop_item, login_string).await
    }
}