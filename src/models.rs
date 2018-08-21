extern crate serde;

use super::schema::account;

use diesel::deserialize::Queryable;


use models::serde::ser::{Serialize, Serializer, SerializeStruct};

use DB;

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
    #[column_name="email_id"]
    pub email      : Option<String>,
}

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
