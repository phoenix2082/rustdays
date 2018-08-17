Step 1 - Now we will be adding support for querying by entity property. For example we can query account by firstname, lastname or email. Let's add support for querying by firstname. For this modify our QueryAccount struct and add another optional property.

pub struct QueryAccount{
    pub offset: u32,
    pub limit: u32,
    pub firstname: Option<String>,
}

Step 2 - Open src/main.rs and modify our Info struct to add optional property support. We need to make firstname property optional as we might not want filter by firstname **always**.

#[derive(Deserialize)]
struct Info {
    pagestart: u32,
    pagesize: u32,
    firstname: Option<String>,
}

Step 3 -  While we are in src/main.rs, let's modify **send** call in our get_accounts_async function to add firstname param to QueryAccount message. I am just showing the send option only, instead of complete function.

....
.send(QueryAccount {
            offset: qparams.pagestart,
            limit: qparams.pagesize,
            firstname: qparams.firstname.clone(),
})
....

Step 4 - Next step is to modify our QueryAccount message handler in **src/accounts.rs**. As we have to make firstname parameter optional when querying database, we have to use into_boxed function provided by diesel query dsl. it allows us to modify query conditionally. In our case, we want to filter result only when firstname param is present. here:

   4.1 - First we are going to call account.into_boxed() and store it in mutable  query variable.
   4.2 - Second we are going to use 'if let' clause to modifu query only when  firstname is present.

The method will look lile below after modification.

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

Step 5: Save your changes and excute following command to start the application.

 $ cargo run 

You should see message like:

 INFO 2018-08-17T09:08:02Z: actix_web::server::srv: Starting 4 http workers
 INFO 2018-08-17T09:08:02Z: actix_web::server::srv: Starting server on http://127.0.0.1:57081
 Started htp server: 127.0.0.1:57081

Step 6: Use curl to filter account's by name.

curl -i http://127.0.0.1:57081/app1/maccounts?pagestart=0\&pagesize=20\&firstname=Captain
HTTP/1.1 200 OK
content-length: 102
content-type: application/json
date: Fri, 17 Aug 2018 09:09:25 GMT

[{"id":1,"firstname":"Captain","middlename":"What","lastname":"America","email":"captain@shield.com"}]

Step 7: Also verify that Account Rest service still returns result when firstname is not used.

curl -i http://127.0.0.1:57081/app1/maccounts?pagestart=0\&pagesize=20
HTTP/1.1 200 OK
content-length: 322
content-type: application/json
date: Fri, 17 Aug 2018 09:12:33 GMT

[{"id":1,"firstname":"Captain","middlename":"What","lastname":"America","email":"captain@shield.com"},{"id":2,"firstname":"Natasha","middlename":"BlackWidow","lastname":"Romanova","email":"blackwidow@shield.com"},{"id":3,"firstname":"T","middlename":"Black Panther","lastname":"Challa","email":"blackpanther@shield.com"}]
