# Adding support for Deleting account by id in Account Rest service.#

If you followed throught all the guide tutorial, then delete request is simplest to implement. For this:

**Step 1** - Let's add a Struct to handle delete messages.

```rust
pub struct DeleteAccount {
    pub id: u32,
}
```

**Step 2** -Then, implement actix::Message type for DeleteAccount struct. In the result we will be only returning whether delete was successful or not. Hence Result should have bool type.

```rust
impl Message for DeleteAccount {
    type Result = Result<bool, Error>;
}
```

**Step 3** - Now we need to implement handler to manage DeleteMessage.

```rust
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
```

**Step 4** - Let's add a struct which we will be using to send response back.

```rust
#[derive(Serialize)]
pub struct DeleteResult {
    result: bool,
}
```

**Step 5** - Next, we will add a method to handle Delete request.

```rust
/// Delete Account request handler.
fn delete_account(
                 (path, state): (Path<(u32)>, State<AppState>),
) -> FutureResponse<HttpResponse> {

    state
        .db
        .send(DeleteAccount {id: path.into_inner()})
        .from_err()
        .and_then(|res| match res {
            Ok(dresult) => Ok(HttpResponse::Ok().json(DeleteResult{ result : dresult})),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
```

**Step 6** -  Last step is to add mapping for our delete request in main method.

```rust

.scope("/maccounts", |acc_scope| {
    acc_scope
        .resource("", |r| {
	    r.method(http::Method::GET).with(get_accounts_async);
	    r.method(http::Method::POST).with(create_account)
        })
        .resource("/{account_id}", |r| {
	    r.method(http::Method::PUT).with(update_account);
            r.method(http::Method::DELETE).with(delete_account)
    })
})

```

**Step 7** - That's it start application using "cargo run" and send delete request as follows:

$ curl -i --request DELETE http://127.0.0.1:57081/app1/maccounts/6

HTTP/1.1 200 OK
content-length: 15
content-type: application/json
date: Tue, 21 Aug 2018 08:09:37 GMT

{"result":true}

You have to replace 6 with id of whatever row you want to delete.

That's it. We can delete an account by id now.
