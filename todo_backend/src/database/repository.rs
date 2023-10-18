use surreal_id::NewId;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use log::debug;

use super::error::{Error, Result};
use crate::model::todo_entry::TodoEntry;

pub const TABLE: &str = "todo_app";
const NAMESPACE: &str = "todo_namespace";
const DB_NAME: &str = "todo_db";
const UNAME: &str = "root"; // add secrets?
const PASS: &str = "rootpassword"; //

pub struct DbOperatorNew {
    address: String,
}

pub struct DbOperatorConnected {
    db: Surreal<Client>,
}

impl DbOperatorNew {
    pub fn new(address: &str) -> Self {
        DbOperatorNew {
            address: address.to_string(),
        }
    }
    pub async fn connect(&self) -> Result<DbOperatorConnected> {
        let db = Surreal::new::<Ws>(&self.address)
            .await
            .map_err(Error::ConnectError)?;
        db.signin(Root {
            username: UNAME,
            password: PASS,
        })
        .await
        .map_err(Error::ConnectError)?;

        // Select a specific namespace / database
        db.use_ns(NAMESPACE)
            .use_db(DB_NAME)
            .await
            .map_err(Error::ConnectError)?;
        Ok(DbOperatorConnected { db })
    }
}

impl DbOperatorConnected {
    pub async fn get_all_records(&self) -> Result<Vec<TodoEntry>> {
        let result: Vec<TodoEntry> = self
            .db
            .select(TABLE)
            .await
            .map_err(Error::ReadRecordError)?;
        Ok(result)
    }
    pub async fn create_record(&self, entry: &TodoEntry) -> Result<()> {
        let _: Vec<TodoEntry> = self
            .db
            .create(TABLE)
            .content(entry)
            .await
            .map_err(Error::CreateRecordError)?;
        debug!(
            "Created record with id: {} !",
            entry.id.id_without_brackets()
        );
        Ok(())
    }
    pub async fn delete_record(&self, entry_id: &str) -> Result<()> {
        let _: Option<TodoEntry> = self
            .db
            .delete((TABLE, entry_id))
            .await
            .map_err(Error::DeleteRecordError)?;
        debug!("Deleting record with id: {} !", entry_id);
        Ok(())
    }
    pub async fn update_record(&self, entry: &TodoEntry) -> Result<()> {
        let _: Option<TodoEntry> = self
            .db
            .update((TABLE, entry.id.id_without_brackets()))
            .content(entry)
            .await
            .map_err(Error::UpdateRecordError)?;
        debug!(
            "Updated record with id: {} !",
            entry.id.id_without_brackets()
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::{collections::HashMap, vec};

    use surreal_id::NewId;

    use crate::database::model::todo_entry::TodoEntry;

    use super::DbOperatorNew;

    use crate::database::error::Result;

    fn init() {
        let _ = env_logger::builder() // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[tokio::test]
    async fn a_test() {
        init();

        clear_db().await.expect("crear db");
        let db = DbOperatorNew::new("localhost:8000");
        let db = db.connect().await.expect("cannot connect to DB");
        let mut non_default_todo = TodoEntry::default();
        let default_todo = TodoEntry::default();
        non_default_todo.priority = 1;
        non_default_todo.title = "non default".to_string();
        non_default_todo.language = "rust".to_string();

        let todos: Vec<TodoEntry> = vec![default_todo.clone(), non_default_todo.clone()];

        for todo in todos {
            db.create_record(&todo).await.expect("error adding entry");
        }

        let all_entries = db.get_all_records().await.expect("cannot get all values");
        let mut hm: HashMap<String, &TodoEntry> = HashMap::new();
        hm.insert(default_todo.id.get_inner_string(), &default_todo);
        hm.insert(non_default_todo.id.get_inner_string(), &non_default_todo);

        for entry in all_entries {
            assert_eq!(**hm.get(&entry.id.get_inner_string()).unwrap(), entry);
        }
        clear_db().await.expect("should clear");
    }
    async fn clear_db() -> Result<()> {
        let db = DbOperatorNew::new("localhost:8000");
        let db = db.connect().await.expect("cannot connect to DB");
        let rec = db.get_all_records().await.expect("should be working");
        for record in rec {
            db.delete_record(&record.id.id_without_brackets())
                .await
                .expect("should be working");
        }
        Ok(())
    }
}
