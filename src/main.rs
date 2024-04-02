#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! Production!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![index]).launch().await?;
    Ok(())
}
