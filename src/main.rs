extern crate actix;
extern crate actix_web;
extern crate customerservice;
extern crate diesel;
extern crate futures;

extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate r2d2;

use actix::Addr;
use actix::SyncArbiter;

use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpResponse;
use actix_web::Path;
use actix_web::Query;
use actix_web::State;

use actix_web::server;
use actix_web::App;
use actix_web::Json;
use actix_web::http;
use actix_web::middleware::Logger;

use customerservice::accounts;
use futures::Future;

use accounts::DbExecutor;
use accounts::QueryAccount;
use accounts::CreateAccount;

use diesel::r2d2::ConnectionManager;
use diesel::prelude::PgConnection;

use customerservice::models::Account;

/// State with DbExecutor address
struct AppState {
    db: Addr<DbExecutor>,
}

#[derive(Deserialize)]
struct Info {
    pagestart: u32,
    pagesize: u32,
    firstname: Option<String>,
}

#[derive(Deserialize)]
struct AccountInfo {
    firstname  : String,
    middlename : Option<String>,
    lastname   : String,
    email      : String,
}

/// Method to laod accounts
/// Async get products request handler
fn get_accounts_async(
        (qparams, state): (Query<Info>, State<AppState>),
    ) -> FutureResponse<HttpResponse> {
    // send async `QueryAccount` message to a `DbExecutor`
    state
        .db
        .send(QueryAccount {
            offset: qparams.pagestart,
            limit: qparams.pagesize,
            firstname: qparams.firstname.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(account) => Ok(HttpResponse::Ok().json(account)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn create_account(
                  (info, state): (Json<AccountInfo>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // send async `CreateProduct` message to a `ProductDbExecutor`
    state
        .db
        .send(CreateAccount
              { firstname  : info.firstname.to_string(),
                middlename : info.middlename.clone(),
                lastname   : info.lastname.to_string(),
		email      : info.email.to_string(),
              })
        .from_err()
        .and_then(|res| match res {
            Ok(account) => Ok(HttpResponse::Ok().json(account)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn update_account(
                 (path, info, state): (Path<(u32)>, Json<Account>, State<AppState>),
) -> FutureResponse<HttpResponse> {

    let in_account = Account {
        id: path.into_inner() as i32,
        ..info.into_inner()
    };

    println!("{:#?}", in_account);
    
    state
        .db
        .send(in_account)
        .from_err()
        .and_then(|res| match res {
            Ok(out_account) => Ok(HttpResponse::Ok().json(out_account)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}


fn main() {

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let customer_system = actix::System::new("customers");

    // Configure and start DB Executor actors
    let manager = ConnectionManager::<PgConnection>::new("postgres://ironman:jarvis@localhost/customers");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(12, move || DbExecutor(pool.clone()));

    // Add new server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .middleware(Logger::default())
            .middleware(Logger::new("%a %{User-Agent}i"))
            .prefix("/app1")
            .scope("/maccounts", |acc_scope| {
                acc_scope
                    .resource("", |r| {
                        r.method(http::Method::GET).with(get_accounts_async);
                        r.method(http::Method::POST).with(create_account)
                    })
                    .resource("/{account_id}", |r| {
                        r.method(http::Method::PUT).with(update_account)
                    })        
            })
            
    })
        .bind("127.0.0.1:57081")
        .unwrap()
        .start();

    println!("Started htp server: 127.0.0.1:57081");
    let _ = customer_system.run();

}
