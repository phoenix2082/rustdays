# rustdays

# Buidling REST service in rust using actix-web, diesel and PostgreSQL. #

## Asumption ##
1. You have completed rust - "The Book".
2. Cargo is configured in you environment.

- [ ] Improve Documentation.
- [ ] Add Test Cases
- [ ] Add support for creatin, update and delete request.

**Step 1:** - Create new cargo project using:

# cargo new --bin customerservice

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
```
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
```
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
 ```       
 
 It is good practice to run cargo check frequently and as there are few module have been added, so we need to add required crates and use statements.
``` 
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
```

**Step 10** - Run cargo check/build to verify if code compiles successfully without any warnings/errors.
