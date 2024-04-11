#[macro_use]
extern crate rocket;

const PORT_NUMBER: u16 = 5000;

mod models;
mod routes;
mod minimax;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            reqwest::Client::builder()
                .user_agent("reqwest")
                .build()
                .unwrap(),
        )
        .configure(rocket::Config::figment().merge(("port", PORT_NUMBER)))
        .mount("/project", routes![routes::index, routes::get_connect_move, routes::get_otto_move])
}