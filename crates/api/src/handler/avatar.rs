use crate::libs::{
    http::AppState,
    redis::{get_value, set_value},
};
use actix_web::{get, web, Error, HttpResponse};
use gen_avatar_lib::{AvatarBuilder, AvatarResult};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, serde::Deserialize)]
pub struct AvatarParams {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AvatarQuery {
    pub theme: Option<String>,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn avatar(name: String, font_color: String, background_color: String) -> AvatarResult {
    AvatarBuilder::new(&name)
        .with_font_color(&font_color)?
        .with_background_color(&background_color)?
        .with_font_scale(150.0)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let s = s / 100.0;
    let l = l / 100.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0)
}

fn generate_color(name_hash: u64, theme: &str) -> String {
    // Pick a random color for hue using the hash
    let hue = name_hash as f32 % 360.0;

    // Pick a random saturation
    let saturation = match theme {
        "dark" => 10.0,
        "light" => 62.0,
        _ => 4.0,
    };

    // Pick a random lightness
    let lightness = match theme {
        "dark" => 15.0,
        "light" => 50.0,
        _ => 80.0,
    };

    let background_color = hsl_to_rgb(hue, saturation, lightness);

    format!(
        "#{:02x}{:02x}{:02x}",
        background_color.0 as u8, background_color.1 as u8, background_color.2 as u8
    )
}

#[get("/avatar/{name}")]
pub async fn generate_avatar(
    path: web::Path<AvatarParams>,
    query: web::Query<AvatarQuery>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();
    let param_input = path.into_inner();

    let theme = match query.theme {
        Some(theme) => theme,
        None => "light".to_string(),
    };

    let hash = calculate_hash(&param_input.name);

    let background_color = generate_color(hash, theme.as_str());

    let font_color = match theme.as_str() {
        "dark" => "#ffffff",
        _ => "#ffffff",
    };

    println!(
        "Generated avatar for: {} with theme: {}, {}",
        param_input.name, font_color, background_color
    );

    // Check if the avatar is in the cache
    let cached_avatar = get_value(
        &mut data
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap(),
        format!("avatar:{}:{}", hash, theme).as_str(),
    )
    .await;

    if !cached_avatar.is_empty() {
        return Ok(HttpResponse::Ok()
            .content_type("image/png")
            .body(cached_avatar));
    }

    let avatar_image = avatar(
        param_input.name.clone(),
        font_color.to_string(),
        background_color.to_string(),
    )
    .unwrap();

    let avatar_image = avatar_image.draw();

    let mut buffer = std::io::Cursor::new(Vec::new());
    avatar_image
        .write_to(&mut buffer, image::ImageFormat::Png)
        .unwrap();
    let buffer = buffer.into_inner();

    let _ = set_value(
        &mut data
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap(),
        format!("avatar:{}:{}", hash, theme).as_str(),
        &buffer,
    )
    .await;

    Ok(HttpResponse::Ok().content_type("image/png").body(buffer))
}

pub fn init_avatar_routes(config: &mut web::ServiceConfig) {
    config.service(generate_avatar);
}
