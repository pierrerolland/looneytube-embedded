mod entities;
mod errors;
mod helpers;
mod templates;

#[macro_use]
extern crate rocket;

use crate::entities::category::Category;
use crate::entities::video::Video;
use crate::helpers::base_dir::base_dir;
use handlebars::Handlebars;
use rocket::fs::{relative, FileServer};
use rocket::response::{content, status};
use serde_json::json;

#[get("/")]
fn index() -> content::RawHtml<String> {
    let categories = Category::find_all();

    content::RawHtml(Handlebars::new().render_template(templates::index(), &json!({
        "categories": categories
    })).unwrap())
}

#[get("/<slug>")]
fn category(slug: String) -> Result<content::RawHtml<String>, status::NotFound<Option<&'static str>>> {
    let category = match Category::find_single(slug) {
        Ok(cat) => cat,
        Err(_) => return Err(status::NotFound(None)),
    };

    let videos = Video::find_all(&category);

    Ok(content::RawHtml(Handlebars::new().render_template(templates::category(), &json!({
        "category": category,
        "videos": videos
    })).unwrap()))
}

#[get("/<category_slug>/<video_slug>")]
fn video(
    category_slug: String,
    video_slug: String,
) -> Result<content::RawHtml<String>, status::NotFound<Option<&'static str>>> {
    let category = match Category::find_single(category_slug) {
        Ok(cat) => cat,
        Err(_) => return Err(status::NotFound(None)),
    };

    let video = match Video::find_single(video_slug, &category) {
        Ok(vid) => vid,
        Err(_) => return Err(status::NotFound(None)),
    };

    Ok(content::RawHtml(Handlebars::new().render_template(templates::video(), &json!({
        "video": video
    })).unwrap()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, category, video])
        .mount("/c", FileServer::from(relative!("/static")))
        .mount("/d", FileServer::from(base_dir()))
}
