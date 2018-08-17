extern crate diesel;
extern crate r2d2;
extern crate actix;
extern crate actix_web;

use accounts::actix::Handler;
use accounts::actix::SyncContext;
use accounts::actix::Actor;
use accounts::actix::Message;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::r2d2::{Error};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use models::Account;
use schema::account::dsl::*;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub struct QueryAccount{
    pub offset: u32,
    pub limit: u32,
    pub firstname: Option<String>,
}

impl Message for QueryAccount {
    type Result = Result<Vec<Account>, Error>;
}    

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<QueryAccount> for DbExecutor {
    type Result = Result<Vec<Account>, Error>;

    fn handle(&mut self, msg: QueryAccount, _: &mut Self::Context) -> Self::Result {

        let conn: &PgConnection = &self.0.get().unwrap();

        let mut query = account.into_boxed();
        
        if let Some(fname) = msg.firstname {
            query = query.filter(firstname.eq(fname));
        }
        
        let mut items = query
            .limit(msg.limit as i64)
            .offset(msg.offset as i64)
            .load::<Account>(conn)
            .expect("Error loading accounts.");

        Ok(items)
    }
}    



