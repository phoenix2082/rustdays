1. Next step is to add pagination support in our Account Rest API. For this Open src/accounts.rs Change QueryAccount struct to following:

pub struct QueryAccount{
    pub offset: u32,
    pub limit: u32,
}

  here offset will be starting row from which we want to fetch records and limit will be number of records we need to fetch.
  
2. Now we need to modify our Handler<QueryAccount> to use this two new parameters. We already have msg: QueryAccount param in our handle method. So all we need to do is to add offset() and limit() call in method chain. After modification it should look like:

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

We have to use type casting here because diesel dsl expect i64 type in offset.

3. Now to add support for pagination in our Account Rest service we are going to actix_web::Query struct extractor. When we have multiple Query Params it is good idea to wrap them in a struct. actix_web automatically extract and maps to query parameters to our struct. It provides nicer encapsulation and makes easy to use in our pagination logic. For this let's add a new struct to capture pagestart and pagesize parameter as follows:

#[derive(Deserialize)]
struct Info {
    pagestart: u32,
    pagesize: u32,
}

We have to add following crates for Deserialization to work.

#[macro_use]
extern crate serde_derive;
extern crate serde;


4. We have to modify our get_accounts_async function signature, so that actix_web can extract and inject query parameter to struct defined in previous steps. Let's add one more param to our get_accounts_async function. It should look like:

  fn get_accounts_async(
      (qparams, state): (Query<Info>, State<AppState>),
  ) -> FutureResponse<HttpResponse> {...}

We have to add following use statement:

use actix_web::Query;

5. Now all left to do is to modify send call in get_accounts_async function and use these two params. It should look like below:

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


6. Start your application by running 'cargo run' and use curl to see pagination working:

 $ curl -i http://127.0.0.1:57081/app1/maccounts?pagestart=0\&pagesize=20

  Output should look like:

HTTP/1.1 200 OK
content-length: 102
content-type: application/json
date: Fri, 17 Aug 2018 04:42:24 GMT

[{"id":1,"firstname":"Captain","middlename":"What","lastname":"America","email":"captain@shield.com"},......]
