mod bookstore;

use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use bookstore::{QueryRoot,BookStore,BookStoreSchema};

#[actix_web::main]
async fn main() ->std::io::Result<()>{
    let schema=Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(BookStore::new())
            .finish();

    HttpServer::new(move ||{
        App::new()
        .data(schema.clone())
        .service(web::resource("/").guard(guard::Post()).to(index))
        .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

//execute는 async-graphql의 기능
//into_inner()은 async_graphql_actix_web::Request 를 async_graphql::Request 으로 변환시켜주는 역할
async fn index(schema: web::Data<BookStoreSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}