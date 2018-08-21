# Update Account using PUT Request in Account Rest Service #

Alright let's add one more feature in our Account Rest service which is to update an account. For this:

**Step 1** - We have implement actix::actix::Message type for our Account schema. Open src/accounts.rs file and add following lines at the end.

```rust
impl Message for Account {
    type Result = Result<Account, Error>;
}
```

**Step 2** - Now Updating Account requires changes in our Account schema. Earlier our Account Schema looked like below:

```rust
#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub firstname  : String,
    pub middlename : Option<String>,
    pub lastname   : String,
    pub email      : String,
}
```

Here only middlename was optional, as not everyone have middle name. We were also using account struct for query request only. For performing update we have to make every property optional, as users of our Account Rest service may want to update only firstname or only lastname or may be two property at same time like firstname, lastname or firstname and email. Think of all possible scenarios you can. For this we have to make all properties optional except id. As for now we don't want users to update id. The primary key **id** can be updated too, if you want but for now let's not do it. I'll tell you later how to do it. So let's change Account schema and make properties optional. Open **src/models.rs** file and change type to Option<String> for our Account's fields.

```rust
#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub firstname  : Option<String>,
    pub middlename : Option<String>,
    pub lastname   : Option<String>,
    pub email      : Option<String>,
}
```

**Step 3** - Ae we are using diesel for database operation, we need to also add couple of annotations to our Account struct. 

```rust
#[derive(Queryable)]
#[derive(Identifiable)]
#[derive(AsChangeset)]
#[table_name="account"]
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Account {
    pub id: i32,
    pub firstname  : Option<String>,
    pub middlename : Option<String>,
    pub lastname   : Option<String>,
    pub email      : Option<String>,
}
```


Here 

**Identifiable** trait - means our structs represent fields, which is mapped one-to-one with a row on a database table.

**AsChangeset** trait - Diesel provides two way to update values in row. Either update one columns at time or pass an Instance of struct to update multiple fields at the same time. If fields are None (i.e. no value is passed in Instance, those column values left untouched.)
     
**Deserialize** trait - is used so that out handler method in **src/account.rs** file can convert easily for sending JSON response back.

**Step 4** - If you recall when we added support for querying account, we added annotation #[derive(Queryable)] to our Account struct in **src/models.rs** file. Earlier during query request rows from account table are mapped to struct which in turn deserialized to string. Now we have converted all fields to Option<String> type, we have to implement Queryable trait by ourselves to return string values. If you compile above code without doing so so may get error something like below:

```rust
52 |             .load::<Account>(conn)
   |              ^^^^ the trait `diesel::Queryable<diesel::sql_types::Text, _>` is not implemented for `std::option::Option<std::string::String>`
   |
```

To get it working let's implement Queryable for our Account struct like this:

```rust
impl Queryable<account::SqlType, DB> for Account {
    type Row = (i32, String, Option<String>, String, String);

    fn build(row: Self::Row) -> Self {
        Account {
            id: row.0,
            firstname  : Some(row.1),
            middlename : row.2,
            lastname   : Some(row.3),
            email      : Some(row.4),
        }
    }
}
```

If you run cargo check again one of the error you may get is :

```rust
error[E0412]: cannot find type `DB` in this scope
  --> src/models.rs:25:35
   |
25 | impl Queryable<accounts::SqlType, DB> for Account {
   |                                   ^^ not found in this scope
```

For this we have to add a type in src/lib.rs before where we decared our modules. Add following lines to get rid of this error.

```rust
use diesel::pg::Pg;

type DB = Pg;
```

and add use statement in src/models.rs

```rust
use DB;
```

Alright run cargo check to verify if code compiles successfully.


**Step 5** - Now lets add a hadler for update request in src/accounts.rs file.

```rust
impl Handler<Account> for DbExecutor {
    type Result = Result<Account, Error>;

    fn handle(&mut self, msg: Account, _: &mut Self::Context) -> Self::Result  {

        let conn: &PgConnection = &self.0.get().unwrap();

        let updated_account = diesel::update(account.find(msg.id))
            .set(&msg)
            .get_result::<Account>(conn)?;
        
        Ok(updated_account)
    }
}
```

**Step 6** - Next step is to add Update Account request handler in **src/main.rs** file.

```rust

/// Update account request handler
fn update_account(
                 (path, info, state): (Path<(u32)>, Json<Account>, State<AppState>),
) -> FutureResponse<HttpResponse> {

    let in_account = Account {
        id: path.into_inner() as i32,
        ..info.into_inner()
    };
    
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
```

We have to also add following use statement for successful compilation.

```rust
use actix_web::Path;
use customerservice::models::Account;
```

**Step 7** - Now all we need to do is add request handler in main method which maps to method added in previous step:

```rust
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
```

**Step 8** - Let's create a new account and then update it's middle name.

```bash
 $ curl --header "Content-Type: application/json" --request POST --data '{"firstname": "Tony", "lastname": "Stark", "email" : "ironman@shield.com"}' http://127.0.0.1:57081/app1/maccounts
{"id":9,"firstname":"Tony","middlename":null,"lastname":"Stark","email":"ironman@shield.com"}
```

And let's update record using PUT request:

$ curl --header "Content-Type: application/json" --request POST --data '{"firstname": "Tony", "lastname": "Stark", "email" : "ironman@shield.com"}' http://127.0.0.1:57081/app1/maccounts

{"id":9,"firstname":"Tony","middlename":null,"lastname":"Stark","email":"ironman@shield.com"}

That's it. We are good for today.
