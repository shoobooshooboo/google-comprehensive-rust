use std::sync::mpsc::{self, Receiver, SyncSender};
use reqwest::Url;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use thiserror::Error;
use std::thread;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand, sender: &SyncSender<Url>) -> Result<(), Error> {
    println!("Checking {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    //let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(());
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                sender.send(link_url);
            }
            Err(err) => {
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    Ok(())
}

fn main() {
    let (inputSender, inputReceiver) = mpsc::sync_channel(100);
    let (outputSender, outputReceiver) = mpsc::sync_channel(100);
    let client = Client::new();
    let start_url = Url::parse("https://www.google.org").unwrap();
    let crawl_command = CrawlCommand{ url: start_url, extract_links: true };
    match visit_page(&client, &crawl_command, &inputSender) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }

    drop(inputSender);
    for url in inputReceiver.iter(){
        let c = client.clone();
        let s = outputSender.clone();
        thread::spawn(move || visit_page(&c, &CrawlCommand{url: url, extract_links: true}, &s));
    }

    drop(outputSender);

    for url in outputReceiver.iter(){
        println!("{}", url.to_string());
    }
}