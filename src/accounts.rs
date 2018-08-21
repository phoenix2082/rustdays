extern crate diesel;
extern crate r2d2;
extern crate actix;
extern crate actix_web;

use accounts::actix::Handler;
use accounts::actix::SyncContext;
use accounts::actix::Actor;
use accounts::actix::Message;

use diesel::r2d2::{ConnectionManager, Pool};
//use diesel::r2d2::{Error};
use diesel::result::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use models::Account;
use schema::account::dsl::*;
use super::schema::account;

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

#[derive(Insertable)]
#[table_name="account"]
pub struct CreateAccount {
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[column_name = "email_id"]
    pub email: String,
}

impl Message for CreateAccount {
    type Result = Result<Account, Error>;
}

impl Handler<CreateAccount> for DbExecutor {
    type Result = Result<Account, Error>;

    fn handle(&mut self, msg: CreateAccount, _: &mut Self::Context) -> Self::Result {

        let conn: &PgConnection = &self.0.get().unwrap();

        let inserted_id: i32 = diesel::insert_into(account)
            .values(&msg)
            .returning(id)
            .get_result(conn)
            .expect("Error creating account");

        let mut items = account
            .filter(id.eq(&inserted_id))
            .load::<Account>(conn)
            .expect("Error loading account after creating.");

        Ok(items.pop().unwrap())
    }
}

impl Message for Account {
    type Result = Result<Account, Error>;
}

impl Handler<Account> for DbExecutor {
    type Result = Result<Account, Error>;

    fn handle(&mut self, msg: Account, _: &mut Self::Context) -> Self::Result  {

        let conn: &PgConnection = &self.0.get().unwrap();

        let updated_account = diesel::update(account.find(msg.id))
            .set(&msg)
            .get_result::<Account>(conn)?;
//            .get_result(conn)?;
        
        Ok(updated_account)
    }
}

pub struct DeleteAccount {
    pub id: u32,
}

impl Message for DeleteAccount {
    type Result = Result<bool, Error>;
}

impl Handler<DeleteAccount> for DbExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: DeleteAccount, _: &mut Self::Context) -> Self::Result  {

        let conn: &PgConnection = &self.0.get().unwrap();

        let num_deleted = diesel::delete(account.filter(id.eq(msg.id as i32)))
            .execute(conn)
            .expect("Error deleting posts");
        
        Ok(if num_deleted == 1 { true } else { false })
    }
}



