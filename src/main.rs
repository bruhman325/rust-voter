// this is a spam voter thing i made in python for mr ellis converted to rust
use std::{fmt, thread, time::{self, Duration}};
use ureq::Agent; // i tried using reqwests but it always would only show the first line of the response and i've had enough
use std::error::Error;
use chrono::Utc;
use std::io::ErrorKind;

const START_VOTE_URL: &str = "https://polls.polldaddy.com/vote-js.php?p=15908244&b=0&a=70042441,&o=&va=16&cookie=0&tags=15908244-src:poll-embed&n=";
const END_VOTE_URL: &str = "&url=https%3A//www.usatodaynetworkservice.com/tangstatic/html/nokl/sf-q1a2z3584c02f3.min.html";

const START_SESSION_URL: &str = "https://poll.fm/n/40024646678ad5de5ee52dc067eb0b79/15908244?";
const END_SESSION_URL: &str = "=";



#[tokio::main]
async fn main() {
    let mut config = Agent::config_builder().build();
    let agent: Agent = config.into();

    

    let mut votes: i32 = 0;

    while true {
        thread::sleep(time::Duration::from_secs(1));
        let mut session = get_session(&agent).await.unwrap();
        match vote(&session, &agent).await {
            Ok(()) => {
                votes = votes + 1;            
            },
            Err(er) => {
                if let Some(session_expired) = er.downcast_ref::<SessionExpiredError>() {
                    println!("session expired");
                    session = get_session(&agent).await.unwrap();
                } else if let Some(ratelimited) = er.downcast_ref::<RateLimitedError>() {
                    println!("rate limited");
                    thread::sleep(Duration::from_secs(60));                    
                }
            },
        }
        println!("{}",votes);
    }

    

}

async fn get_session(agent: &Agent) -> Result<String,Box<dyn std::error::Error>> {
    let request: String = format!("{}{}{}",START_SESSION_URL,Utc::now().timestamp(),END_SESSION_URL);

    let data = agent.get(request).call()?.body_mut().read_to_string()?;

    

    let start_session = data.find("'").unwrap(); 
    let end_session = &data[start_session+1..].find("'").unwrap();

    

    Ok((String::from(&data[start_session+1..start_session+end_session+1])))
}

async fn vote(session: &String, agent: &Agent) -> Result<(),Box<dyn std::error::Error>> {
    let request: String = format!("{}{}{}",START_VOTE_URL,session,END_VOTE_URL);

    let data = agent.get(request).call()?.body_mut().read_to_string()?;

    println!("{}",data);
    if data.len() < 200 {        
        return Err(Box::new(SessionExpiredError))
    }

    if !data.find("already-registered").is_none() {
        println!("rate limited");
        return Err(Box::new(RateLimitedError))
    }

    

    Ok(())
}

#[derive(Debug,Clone, PartialEq)]
struct SessionExpiredError;

#[derive(Debug,Clone, PartialEq)]
struct RateLimitedError;

impl fmt::Display for SessionExpiredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"session expired")
    }
}

impl fmt::Display for RateLimitedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"rate limited")
    }
}

impl Error for SessionExpiredError {}
impl Error for RateLimitedError {}