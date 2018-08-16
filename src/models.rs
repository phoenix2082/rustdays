extern crate serde;

use models::serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub firstname  : String,
    pub middlename : Option<String>,
    pub lastname   : String,
    pub email      : String,
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
