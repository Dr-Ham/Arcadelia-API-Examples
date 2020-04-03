
use reqwest;
use tokio;
use serde_json::{Value};
use std::error::Error;

/*
Output:
PROFILE API EXAMPLE:
 DrHam2 (SA,USER): Develops webs
*/
async fn api_call_example_profile(name: String) -> Result<(), reqwest::Error> {

   let url = format!("http://127.0.0.1:8000/api/user/byname/{}", name);
   let content = reqwest::get(&url).await?.text().await?;

   let user: Value = serde_json::from_str(&content).unwrap();
   let response = format!("{username} ({role}): {bio}", username = user["username"], role = user["roles"]["roles"], bio = user["bio"])
       .replace("\"", "").replace("ROLE_", "").replace("[", "").replace("]", "");
   println!("PROFILE API EXAMPLE: \n {}", response);

    Ok(())
}

/*
Output:
PROFILE API EXAMPLE:
 Appeals: 3
 Bug Reports: 0
 Reports 3
 Users 3
*/
async fn api_call_example_stats() -> Result<(), reqwest::Error> {

   let url = format!("http://127.0.0.1:8000/api/stats");
   let content = reqwest::get(&url).await?.text().await?;

   let stats: Value = serde_json::from_str(&content).unwrap();
   let response = format!("Appeals: {appeals} \n Bug Reports: {bugs} \n Reports {reports} \n Users {users} \n ",
        appeals = stats["Appeals"], bugs = stats["BugsReports"], reports = stats["Reports"], users = stats["Users"] )
       .replace("\"", "");
   println!("STATS API EXAMPLE: \n {}", response);

   Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
   let username: String = "DrHam2".to_string();
   api_call_example_profile(username).await?;
   api_call_example_stats().await?;
   Ok(())
}
