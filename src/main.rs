pub mod positions;
pub mod rules;
pub mod tables;
pub mod states;

use rocket::{catch, catchers, get, launch, routes};
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use crate::positions::{Point2D, Position};
use crate::rules::Rule;
use crate::states::StandardStates;
use crate::tables::{Table, UnlimitedGrid};



#[get("/classic/<n_generations>", data = "<data>")]
fn play_classic(n_generations: u32,  data: Json<Vec<Point2D>>) -> Json<Vec<Vec<Point2D>>> {

    let points = data.into_inner();

    let mut initial_table: UnlimitedGrid<Point2D, StandardStates> = UnlimitedGrid::new();
    for point in points {
        initial_table.insert(point)
    }

    let generations: Vec<Box<dyn Table<Point2D, StandardStates>>> = play(
        Box::new(initial_table),
        Box::new(rules::StandardRules),
        n_generations
    );

    let mut result: Vec<Vec<Point2D>> = Vec::new();
    for generation in generations {
        result.push(generation.get_active_cells().iter().map(|(p, _)| p.clone()).collect());
    }

    Json(result)
}



#[catch(404)]
fn not_found() -> Json<String> {
    Json(String::from("Resource not found"))
}

#[launch]
fn rocket() -> _ {

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("error creating CORS fairing");

    rocket::build()
        .mount("/", routes![play_classic])
        .attach(cors)
        .register("/", catchers![not_found])

}





fn play<P, S: 'static>(
    table: Box<dyn Table<P, S>>,
    rule: Box<dyn Rule<S>>,
    n_generations: u32,
) -> Vec<Box<dyn Table<P, S>>> where P: Position + 'static, {
    let mut result = vec![table];
    for _ in 0..n_generations {
        let last_table = result.last().unwrap().clone();
        let next_table = last_table.tick(&rule);
        result.push(next_table);
    }
    result
}