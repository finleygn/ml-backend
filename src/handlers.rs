use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder, get, post, Error, Result};
use juniper::http::{ graphiql::graphiql_source, GraphQLRequest};

use crate::{AppState, graphql::Schema};

#[get("/")]
pub async fn index() ->  impl Responder {
    HttpResponse::Ok().body("Yooo")
}

#[post("/graphql")]
pub async fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let res = data.execute(&schema, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await
        .map_err(Error::from).unwrap();

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(res)
    )
}

#[get("/graphiql")]
pub async fn graphiql() ->  impl Responder {
    let html = graphiql_source("/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn not_found() ->  impl Responder {
    HttpResponse::NotFound().body("404")
}
