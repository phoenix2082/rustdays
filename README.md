# rustdays

# Buidling REST service in rust using actix-web, diesel and PostgreSQL. #

## Asumption ##
1. You have completed rust - "The Book".
2. Cargo is configured in you environment.

- [ ] Improve Documentation.
- [ ] Add Test Cases
- [ ] Add support for creating, update and delete request.
- [ ] Add more info about what we are going to learn.

## Links ##
[Tutorial - Adding support for pagination in Account Rest Service](https://github.com/phoenix2082/rustdays/blob/master/guide/add-pagination-support.md)

[Tutorial - Adding support for Query by Optional Parameter](https://github.com/phoenix2082/rustdays/blob/master/guide/add-support-for-query-by-optional-parameters.md)

**Step 1:** - Create new cargo project using:
```
$ cargo new --bin customerservice
```
and Add dependencies in Cargo.toml

```
actix = "0.7"
actix-web = "0.7.3"
serde = "1.0"
serde_derive = "1.0"
log = "0.4.0"
env_logger = "0.5.12"
diesel = { version = "^1.1.0", features = ["r2d2", "postgres"] }
r2d2 = "0.8"
dotenv = "0.10"
futures = "0.1.23"
```

**Step 2:** - Run Cargo build command so that it pull all dependencies.

**Step 3:** - Run below command in PROJECT_HOME directory. Below command used to create .env file with database connection URL. Later we will access it using enviornmental variable. **_User must have permission to create database otherwise step 4 will not create database._**

```
$ echo DATABASE_URL=postgres://ironman:jarvis@localhost/customers > .env
```

**Step 4:** - Install diesel cli and Run diesel setup command to create database.
```
 $ cargo install diesel_cli
 $ diesel setup
```

**Step 5:** - Create new migration to create table to store account data.
```
$ diesel migration generate create_customers
```
     
**Step 6:** - Open migration/[DIR_NAME]/up.sql and add create table statement.

```sql
CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR NOT NULL,
  middlename VARCHAR,
  lastname VARCHAR NOT NULL,
  email_id VARCHAR NOT NULL
)
```

**Step 7:** - Open migration/[DIR_NAME]/down.sql and drop statement. 
```
DROP account;
```

**Step 8:** - Run following command to create table:-
```
# diesel migration run
```

**Step 9:** - Create lib.rs file if does not exist in src directory and add following method to connect to database.
```rust
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
 ```       
 
 It is good practice to run cargo check frequently and as there are few module have been added, so we need to add required crates and use statements.
```rust
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
```

**Step 10** - Run cargo check/build to verify if code compiles successfully without any warnings/errors.

**Step 11** - Now, we need to create struct to read data from DB. For this add following two module definition at the top after use statement in lib.rs file.
```rust
pub mod schema;
pub mod models;
```

**Step 12** - Now we need to create two modules which we have just declared in previous step. Proceed to next step.

**Step 13** - Create model.rs file next to lib.rs file. And create a struct to map table data. The #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query. Example -
```rust
#[derive(Queryable)]
pub struct Account {
    pub id         : i32,
    pub firstname  : String,
    pub middlename : Option<String>,
    pub lastname   : String,
    pub email      : String,
}
```

**Step 14** - Usually schema.rs file is not created by hand and is automatically generated when **"diesel setup"** command is executed. It is generated in src directory. Open and verify that generated schema meets you expectation of vaious column types. Here is sample generated for me by "diesel setup command". I am using PostgreSQL, if you are using some other db , things might not look same.
```
table! {
  account (id) {
      id -> Int4,
      firstname -> Varchar,
      middlename -> Nullable<Varchar>,
      lastname -> Varchar,
      email_id -> Varchar,
  }
}
```

**Step 15** - It is good practice to keep your entity related methods in separate file. We are going to add new module first and then create required file to keep all functions together. For this add following line after where you have added model and schema module in lib.rs file.
    
    pub mod accounts;

Then create a file called - **_accounts.rs_** next to **_lib.rs_** file.

**Step 16** - We are going to use actix-web to build our rest services. The actix-web is built on top of actix which have very efficient Actor/Messeging support so we wil be using Actix to pass data using Messages to access data. Add following crates at the top of file created in previous steps to add support for required crates.
```rust
extern crate diesel;
extern crate r2d2;
extern crate actix;
extern crate actix_web;
```
Right now is good time to run cargo check to verify if things are working well.

**Step 17** - We are going to define a struct for ConnectionPool. Add following line for this: 

```rust
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);
```

Compiling following lines successfully requires addition of below use statement.

```rust
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use diesel::r2d2::Pool;
```

Run cargo check to see if it compiles successfully.

**Step 18** -Now we need to create another struct for Message passing to query accounts (You can give it a name based on whatever name reflect best mapping to table name. If you have created table/enity with some other name.)

```rust
pub struct QueryAccount;
```

**Step 19** - Implement Message holder for QueryAccount struct created in previous step, which is actually used by actix as valid message.

```rust
impl Message for QueryAccount {
    type Result = Result<Vec<Account>, Error>;
}
```

We need to add following use statements to make above code compiles successfully.

```rust 
use accounts::actix::Message;
use diesel::r2d2::Error;
```

Run cargo build again to make sure changes made so far compiles.

**Step 20** - Finally we need to implement an Actor, which will be spawned by actix Actor system whenever a new request arrives query an account.

```rust
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
```

We need to add following use statements to make above code compiles successfully.

```rust
use accounts::actix::Actor;
use accounts::actix::SyncContext;
```

Run cargo check/build again to make sure changes made so far compiles.

**Step 21** - Now we are going to implement a handler which is actually going to query the database to get list of accounts.

```rust
impl Handler<QueryAccount> for DbExecutor {
    type Result = Result<Vec<Account>, Error>;

    fn handle(&mut self, msg: QueryAccount, _: &mut Self::Context) -> Self::Result {

        let conn: &PgConnection = &self.0.get().unwrap();

        let mut items = account
            .load::<Account>(conn)
            .expect("Error loading accounts.");

        Ok(items)
    }
}
```

We need to add following use statements to make above code compiles successfully.

```rust
use accounts::actix::Handler;
use models::Account;
use schema::account::dsl::*;
```

**Step 22** - Now we are going to add request handler in **_main.rs_** file, which we be mapped to URI later. For this first we need to create a State holder for DBExecutor we created in previous steps.

```rust
/// State with DbExecutor address
struct AppState {
    db: Addr<DbExecutor>,
}
```

We need to add following use statements to make above code compiles successfully.

```rust
use accounts::DbExecutor;
```

**Step 23** - Now we are going to add method which is going to use DbExecutor state to handle incoming QueryAccount Message and return list of accounts as JSON.

```rust
/// Method to load accounts.
/// Async get accounts request handler
fn get_accounts_async(state: State<AppState>) -> FutureResponse<HttpResponse> {
    // send async `QueryAccount` message to a `DbExecutor`
    state
        .db
        .send(QueryAccount)
        .from_err()
        .and_then(|res| match res {
            Ok(account) => Ok(HttpResponse::Ok().json(account)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
```

We have to add following crates and use statement to compile code successfully.

```rust
extern crate actix_web;
extern crate customerservice;
extern crate futures;
```

Add it after other use statements added so far.

```rust
use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpResponse;
use actix_web::State;
use customerservice::accounts;
use futures::Future;
```

We won't be getting any error related to missing functions or modules, but we will be getting error that account can not be converted as Accont struct is not serializble. Move to next step to resolve this error

**Step 24** - Implement Serializer for Account struct so that it can be converted to JSON while sending response back to query account request. This needs to be done in **_accounts.rs_** file.

```rust

extern crate serde;

use models::serde::ser::{Serialize, Serializer, SerializeStruct};

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Account", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("firstname", &self.firstname)?;
        state.serialize_field("middlename", &self.middlename)?;
        state.serialize_field("lastname", &self.lastname)?;
        state.serialize_field("email", &self.email)?;
        state.end()
    }
}
```

**Step 25** - Now the basic work has been done i.e. getting data from DB and converting it to JSON. Next step is to map it to request URI. This is done using actix-web. Open **_main.rs_** file and add following lines:

```rust
std::env::set_var("RUST_LOG", "actix_web=info");
env_logger::init();
```

We have just initialized a logger by adding previous two lines and set log level to INFO. You have to add following two crates if not already done so.

```rust
extern crate log;
extern crate env_logger;
```

**Step 26**: Next we are going to create our Actor System which will be handling all messages. This can be done with one line only:

```rust
let product_system = actix::System::new("products");
```

**Step 27**: We will create, configure and start our DB Executor Actors which is expected to handle all incoming messages using pool.

```rust
// Configure and start DB Executor actors
let manager = ConnectionManager::<PgConnection>::new("postgres://ironman:jarvis@localhost/customers");
let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

let addr = SyncArbiter::start(12, move || DbExecutor(pool.clone()));
```

Following crates and use statement need to be added to compile/build the above code.

```rust
extern crate diesel;
extern crate r2d2;

use actix::SyncArbiter;

use diesel::r2d2::ConnectionManager;
use diesel::prelude::PgConnection;
```

**Step 28**: Now we can use Actix Web to create new server and register app with handler to serve Query Account Requests.

```rust
// Add new server
server::new(move || {
    App::with_state(AppState{db: addr.clone()})
        .middleware(Logger::default())
        .middleware(Logger::new("%a %{User-Agent}i"))
        .prefix("/app1")    
        .resource("/maccounts",
                  |r| r.method(http::Method::GET).with(get_accounts_async))
})
  .bind("127.0.0.1:57081")
  .unwrap()
  .workers(1)
  .start();
```

we need to add following crates and use statements to get above code compiles successfully.

```rust
extern crate log;

use actix_web::server;
use actix_web::App;
use actix_web::middleware::Logger;
use actix_web::http;
use actix_web::http::Method;
```

**Step 29**: Now we can add statement to run the Actor system..

```
println!("Started htp server: 127.0.0.1:57081");
let _ = customer_system.run();
```

**Step 30**: Execute cargo run command to start the server. You should see soomething like below:

```
 INFO 2018-08-16T13:32:08Z: actix_web::server::srv: Starting 4 http workers
 INFO 2018-08-16T13:32:08Z: actix_web::server::srv: Starting server on http://127.0.0.1:57081
```

**Step 31**: Use **curl** to query accounts. The output should look something like below:

```
$ curl -i http://127.0.0.1:57081/app1/maccounts

HTTP/1.1 200 OK
content-length: 2
content-type: application/json
date: Thu, 16 Aug 2018 12:07:43 GMT
```
Hurray our server is running and telling us it's doing okay with "200 OK" message.

**Step 32** - Now we need to create some data to actually see some actual JSON response. Create some records using below statement if you are using PostgreSQL.

```sql
 INSERT INTO account (firstname, middlename, lastname, email_id) VALUES ('Captain', 'What!', 'America', 'captain@shields.com');
 ```
 
**Step 33** - Curl again and you should see some response this time.

```
curl -i http://127.0.0.1:57081/app1/maccounts
HTTP/1.1 200 OK
content-length: 102
content-type: application/json
date: Thu, 16 Aug 2018 12:13:04 GMT

[{"id":1,"firstname":"Captain","middlename":"What","lastname":"America","email":"captain@shield.com"}]
```

Alrigth that's enough for today.

 

