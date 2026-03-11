use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
}

fn main() {
    println!("🔶 Top 10 Hacker News Stories\n");

    let client = reqwest::blocking::Client::new();
    let mut pass = false;
    let mut response = String::new();
    let mut  url = String::new();
    while !pass
    {
        print!("Do you wnat to see Top, New, Best or Ask");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        if response.trim() == "Top"
        {
            url = "https://hacker-news.firebaseio.com/v0/topstories.json".to_string();
            pass = true;
        }
        else if response.trim() == "New"
        {
            url = "https://hacker-news.firebaseio.com/v0/newstories.json".to_string();
        }
        else if response.trim() == "Best"
        {
            url = "https://hacker-news.firebaseio.com/v0/beststories.json".to_string();
            pass = true;
        }
        else if response.trim() == "Ask"
        {
            url = "https://hacker-news.firebaseio.com/v0/askstories.json".to_string();
            pass = true;
        }
        else 
        {
            pass = false;
        }
    }
    
    let top_ids: Vec<u64> = client
        .get(url)
        .send()
        .expect("Failed to fetch top stories")
        .json()
        .expect("Failed to parse story IDs");

    for (i, id) in top_ids.iter().take(10).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        let link = story.url.as_deref().unwrap_or("(no URL)");
        println!("{}. {} ({} points by {})", i + 1, story.title, story.score, story.by);
        println!("   {}\n", link);
    }
}