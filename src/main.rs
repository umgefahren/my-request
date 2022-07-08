use std::borrow::Cow;
use actix_web::http::{StatusCode, Version};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, HttpMessage};
use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
		tera.add_raw_template("index.html", include_str!("../templates/index.html")).expect("Couldn't parse html template");
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

fn version_to_str(version: Version) -> &'static str {
    match version {
        Version::HTTP_09 => "HTTP/0.9",
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2",
        Version::HTTP_3 => "HTTP/3",
        _ => "unknown",
    }
}

async fn index(req: HttpRequest) -> impl Responder {
    let mut context = Context::new();
    let mut headers: Vec<(&str, Cow<'_, str>)> = req.headers()
        .iter()
        .map(|(name, value)| (name.as_str(), String::from_utf8_lossy(value.as_bytes())))
        .collect();

    headers.sort();

    match req.connection_info().realip_remote_addr() {
        Some(d) => context.insert("remote", d),
        _ => context.insert("remote", "NaN"),
    }


    let cookies = match req.cookies() {
        Ok(m) => m.iter().map(|e| (e.name().to_string(), e.value().to_string())).collect(),
        _ => vec![],
    };

    context.insert("cookies", &cookies);

    context.insert("version", version_to_str(req.version()));

    context.insert("path", req.path());

    context.insert("contentType", req.content_type());

    context.insert("headers", &headers);

    let body = TEMPLATES.render("index.html", &context).unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().default_service(actix_web::web::to(index)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
