## Adding Support for Creating Accounts in "Account Rest Service".

Next feature we want is to have ability to create accounts using our Account Rest Service. We are still going to use Actor models. Our Actors will accept message to Create an Account, Send request to DB and create a record and return the Account when Account is created successfully.

**Step 1** - Let's create a Struct to declare a holder for Account's proeprties.

```rust
#[derive(Insertable)]
#[table_name="account"]
pub struct CreateAccount {
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[column_name = "email_id"]
    pub email: String,
}
```

**Step 2** - Next step is to implement Message behaviour for Creating Account.

```rust
impl Message for CreateAccount {
    type Result = Result<Account, Error>;
}
```

**Step 3** - Now we are going to implement Message handler which will do the actual job of inserting a record in account table and return a account with id when created successfully.

```rust
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
```

**Step 4** - Now in src/main.rs we need to add struct, which will be used by JSON extractor to map values from request and create a message to call backend handler.

```
#[derive(Deserialize)]
struct AccountInfo {
    firstname  : String,
    middlename : Option<String>,
    lastname   : String,
    email      : String,
}
```

**Step 5** - Now we have to add a new function in src/main.rs which will handle the create Account request, extract the json from request body, create a message and pass it to backend handler.

```
/// Create account request handler
fn create_account(
                  (info, state): (Json<AccountInfo>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // send `CreateAccount` message to a `DbExecutor`
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
```

We have to add following use statement to build our code successfully.

```rust
use actix_web::Json;

use accounts::CreateAccount;
```

**Step 6** - Next step is to map our method with create account POST request. For this add following lines to our App in main method.

```rust

// Add new server
   ...
   server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .middleware(Logger::default())
            .middleware(Logger::new("%a %{User-Agent}i"))
            .prefix("/app1")    
            .resource("/maccounts", |r| {
                r.method(http::Method::GET).with(get_accounts_async);
                r.method(http::Method::POST).with(create_account)
            })
            
    })
    ...
    
 ```   

That's it. open terminal and execute **cargo run** to fire up server.

**Step 7** - Lets create our first account using curl.

```
 curl -i --header "Content-Type: application/json" --request POST --data '{"firstname": "Pietro", "middlename": "Quicksilver", "lastname" : "Maximoff", "email": "quicksilver@shield.com"}' http://127.0.0.1:57081/app1/maccounts
HTTP/1.1 200 OK
content-length: 111
content-type: application/json
date: Sun, 19 Aug 2018 07:14:40 GMT

{"id":7,"firstname":"Pietro","middlename":"Quicksilver","lastname":"Maximoff","email":"quicksilver@shield.com"}
```


That's it. We have added support for creating accounts.
