#[macro_use] extern crate rocket;

use anyhow::{Context, Error};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use rocket::{response::{Redirect, Debug}, http::{CookieJar, Cookie, SameSite, hyper::Response}};
use rocket_oauth2::{OAuth2, TokenResponse};

struct Discord;

#[get("/login/discord")]
fn discord_login(oauth2: OAuth2<Discord>, mut cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["identify"]).unwrap()
}

/// User information to be retrieved from the GitHub API.
#[derive(serde::Deserialize)]
struct DiscordUserInfo {
    id: String,
    username: String,
    avatar: String,
    global_name: String
}


#[get("/auth/discord")]
async fn discord_callback(token: TokenResponse<Discord>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<Error>> {
    // Set a private cookie with the access token
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish()
    );

    let disrequest = reqwest::Client::builder()
        .build()
        .context("Failed to build request :(")
        .unwrap()
        .get("https://discord.com/users/@me")
        .header(AUTHORIZATION, format!("token {}", token.access_token()))
        .header(USER_AGENT, "deadbirdmark v1.0")
        .send()
        .await
        .context("Failed while waiting")
        .unwrap();

    println!("{}", disrequest.text().await.context("a").unwrap());

    // let user_info: DiscordUserInfo = disrequest.json().await.context("Failed to get JSON").unwrap();
    // println!("User: {}", user_info.id);


    // cookies.add_private(
    //     Cookie::build("user_id", user_info.id)
    //     .same_site(SameSite::Lax)
    //     .finish()
    // );
    // cookies.add_private(
    //     Cookie::build("user_name", user_info.username)
    //     .same_site(SameSite::Lax)
    //     .finish()
    // );
    // cookies.add_private(
    //     Cookie::build("user_avatar", user_info.avatar)
    //     .same_site(SameSite::Lax)
    //     .finish()
    // );

    Ok(Redirect::to("/"))
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, discord_callback, discord_login])
    .attach(OAuth2::<Discord>::fairing("discord"))
}