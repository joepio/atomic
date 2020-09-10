use crate::{appstate::AppState, content_types::get_accept};
use crate::{
    content_types::ContentType,
    errors::BetterResult,
    render::propvals::{from_hashmap_resource, PropVal},
};
use actix_web::{http, web, HttpResponse};
use serde::Deserialize;
use std::sync::Mutex;
use tera::Context as TeraCtx;
use atomic_lib::Storelike;

#[derive(Deserialize, Debug)]
pub struct GetQuery {
    path: Option<String>,
}

pub async fn path(
    data: web::Data<Mutex<AppState>>,
    query: web::Query<GetQuery>,
    req: actix_web::HttpRequest,
) -> BetterResult<HttpResponse> {
    let path = &query.path.clone().unwrap_or_default();
    let context = data.lock().unwrap();
    let content_type = get_accept(req);

    log::info!("path: {:?}", path);
    // This is how locally items are stored (which don't know their full subject URL) in Atomic Data
    let mut builder = HttpResponse::Ok();
    let path_result = context.store.get_path(&path, Some(&context.mapping))?;
    match content_type {
        ContentType::JSON => {
            builder.set(http::header::ContentType::json());
            //   let body = context.store.resource_to_json(&subject, 1)?;
            Ok(builder.body("Not implemented"))
        }
        ContentType::JSONLD => {
            builder.set(http::header::ContentType::json());
            //   let body = context.store.resource_to_json(&subject, 1)?;
            Ok(builder.body("Not implemented"))
        }
        ContentType::HTML => {
            let mut propvals: Vec<PropVal> = Vec::new();
            match path_result {
                atomic_lib::storelike::PathReturn::Subject(subject) => {
                    let resource = context
                        .store
                        .get_resource_string(&subject)?;
                    propvals = from_hashmap_resource(&resource, &context.store, subject)?;
                }
                atomic_lib::storelike::PathReturn::Atom(atom) => {
                    propvals.push(PropVal {

                        property: context.store.get_property(&atom.property.subject)?,
                        value_html: crate::render::atom::value_to_html(&atom.native_value),
                        value: atom.value,
                        subject: atom.subject,
                    });
                }
            }
            builder.set(http::header::ContentType::html());
            let mut tera_context = TeraCtx::new();
            tera_context.insert("propvals", &propvals);
            tera_context.insert("path", &path);
            let body = context.tera.render("path.html", &tera_context).unwrap();
            Ok(builder.body(body))
        }
        ContentType::AD3 => {
            builder.set(http::header::ContentType::html());
            //   let body = context.store.resource_to_ad3(&subject, Some(&context.config.domain))?;
            Ok(builder.body("Not implemented"))
        }
    }
}
