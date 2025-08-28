// this is a spam voter thing i made in python for mr ellis converted to rust 
// https://www.oklahoman.com/story/sports/high-school/football/2025/08/19/vote-who-is-best-oklahoma-high-school-defensive-lineman-for-2025/85703398007/?authuser=0
// use std::{fmt, thread, time::{self, Duration}};
// use ureq::Agent; // i tried using reqwests but it always would only show the first line of the response
// use std::error::Error;
// use chrono::Utc;
// use std::io::ErrorKind;
mod voting_module;
pub use crate::voting_module::voting_helper;
use ureq::Agent; // i tried using reqwests but it always would only show the first line of the response
use std::thread;
use std::time;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut config = Agent::config_builder().build();
    let agent: Agent = config.into();

    let mut votes: i32 = 0;

    while true {
        thread::sleep(time::Duration::from_secs(1));
        let mut session = voting_helper::get_session(&agent).await.unwrap();
        match voting_helper::vote(&session, &agent).await {
            
            Ok(()) => {
                votes = votes + 1;            
            },

            Err(er) => {
                if let Some(_session_expired) = er.downcast_ref::<voting_helper::SessionExpiredError>() {
                    println!("session expired");
                    session = voting_helper::get_session(&agent).await.unwrap();
                } else if let Some(_ratelimited) = er.downcast_ref::<voting_helper::RateLimitedError>() {
                    println!("rate limited");
                    thread::sleep(Duration::from_secs(60));                    
                }
            },
            
        }
        println!("{} votes",votes);
    }
}

