
**Step 1** - Now we are going to add pagination support in our Account Rest API. For this Open src/accounts.rs file and change QueryAccount struct to following:

```
pub struct QueryAccount {
    pub offset: u32,
    pub limit: u32,
}
```

  Here offset will be starting row from which we want to fetch records from database and limit will be number of records we need to fetch.
  
**Step 2** - Now we need to modify our Handler<QueryAccount> function to use these two new parameters. We already have **_msg: QueryAccount_** parameter in our handle method parameter list. So all we need to do is to add offset() and limit() call in method chain. After modification it should look like as follows:
    
```
impl Handler<QueryAccount> for DbExecutor {
    type Result = Result<Vec<Account>, Error>;

    fn handle(&mut self, msg: QueryAccount, _: &mut Self::Context) -> Self::Result {

        let conn: &PgConnection = &self.0.get().unwrap();

        let mut items = account
            .limit(msg.limit as i64)
            .offset(msg.offset as i64)
            .load::<Account>(conn)
            .expect("Error loading accounts.");

        Ok(items)
    }
}    
```
We have to use type casting here because diesel dsl expect i64 type in offset.

**Step 3** - Now to add support for pagination in our Account Rest service we are going to use **actix_web::Query** extractor. When we have multiple Query Parameters, it is a good idea to wrap them in a struct. The actix_web automatically extract and maps to query parameters to our struct. It provides nicer encapsulation and makes easy to use in our pagination logic. For this let's add a new struct to capture pagestart and pagesize parameter as follows:

```
#[derive(Deserialize)]
struct Info {
    pagestart: u32,
    pagesize: u32,
}
```

We have to add following crates for Deserialization to work.
```
#[macro_use]
extern crate serde_derive;
extern crate serde;
```

**Step 4** - We have to modify our get_accounts_async function signature in **src/main.rs** file now, so that actix_web can extract and inject query parameter to struct defined in previous steps. Let's add one more parameter to our get_accounts_async function. It should look like:

```
fn get_accounts_async(
    (qparams, state): (Query<Info>, State<AppState>),
) -> FutureResponse<HttpResponse> {...}
```

We have to add following use statement:

```
use actix_web::Query;
```

**Step 5** - Now all left to do is to modify send call in get_accounts_async function and use these two parameters. It should look like as follows:

```
fn get_accounts_async(
     (qparams, state): (Query<Info>, State<AppState>),
     ) -> FutureResponse<HttpResponse> {
        // send async `QueryAccount` message to a `DbExecutor`
        state
            .db
            .send(QueryAccount {
                offset: qparams.pagestart,
                limit: qparams.pagesize,
            })
            .from_err()
            .and_then(|res| match res {
                Ok(account) => Ok(HttpResponse::Ok().json(account)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
        .responder()
 }
```

**Step 6** - Start your application by running 'cargo run' and use curl to see pagination working:

```
 $ curl -i http://127.0.0.1:57081/app1/maccounts?pagestart=0\&pagesize=20
```

Output should look like:

```
HTTP/1.1 200 OK
content-length: 102
content-type: application/json
date: Fri, 17 Aug 2018 04:42:24 GMT

[{"id":1,"firstname":"Captain","middlename":"What","lastname":"America","email":"captain@shield.com"},......]
```
