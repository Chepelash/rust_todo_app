#![allow(unused)]

use surreal_id::NewId;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use super::error::{Error, Result};
use super::model::todo_entry::TodoEntry;

pub const TABLE: &str = "todo_app";

pub struct DbOperatorNew {
    address: String,
}

struct DbOperatorConnected {
    db: Surreal<Client>,
}

impl DbOperatorNew {
    pub fn new(address: &str) -> Self {
        DbOperatorNew {
            address: address.to_string(),
        }
    }
    async fn connect(&self) -> Result<DbOperatorConnected> {
        let db = Surreal::new::<Ws>(&self.address)
            .await
            .map_err(Error::ConnectError)?;
        db.signin(Root {
            username: "root",
            password: "rootpassword",
        })
        .await
        .map_err(Error::ConnectError)?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await;
        Ok(DbOperatorConnected {
            db,
        })
    }
}

impl DbOperatorConnected {
    async fn get_all_records(&self) -> Result<Vec<TodoEntry>> {
        let result: Vec<TodoEntry> = self
            .db
            .select(TABLE)
            .await
            .map_err(Error::ReadRecordError)?;
        Ok(result)
    }
    async fn create_record(&self, entry: &TodoEntry) -> Result<()> {
        let record: Vec<TodoEntry> = self
            .db
            .create(TABLE)
            .content(entry)
            .await
            .map_err(Error::CreateRecordError)?;
        Ok(())
    }
    async fn delete_record(&self, entry: &TodoEntry) -> Result<()> {
        let record: Option<TodoEntry> = self
            .db
            .delete((TABLE, entry.id.id_without_brackets()))
            .await
            .map_err(Error::DeleteRecordError)?;
        Ok(())
    }
    async fn update_record(&self, entry: &TodoEntry) -> Result<()> {
        let record: Option<TodoEntry> = self
            .db
            .update((TABLE, entry.id.id_without_brackets()))
            .await
            .map_err(Error::UpdateRecordError)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{vec, collections::HashMap, rc::Rc};

    use surreal_id::NewId;

    use crate::database::model::todo_entry::TodoEntry;

    use super::DbOperatorNew;

    use crate::database::error::Result;

    #[tokio::test]
    async fn a_test() {
        clear_db().await.expect("crear db");        
        let db = DbOperatorNew::new("localhost:8000");
        let db = db.connect().await.expect("cannot connect to DB");
        let mut non_default_todo = TodoEntry::default();
        let default_todo = TodoEntry::default();
        non_default_todo.priority = 1;
        non_default_todo.title = "non default".to_string();
        non_default_todo.language = "rust".to_string();
        
        let todos: Vec<TodoEntry> = vec![
            default_todo.clone(),
            non_default_todo.clone()
        ];

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

    }
    async fn clear_db() -> Result<()> {
        let db = DbOperatorNew::new("localhost:8000");
        let db = db.connect().await.expect("cannot connect to DB");
        let rec = db.get_all_records().await.expect("should be working");
        for record in rec {
            db.delete_record(&record).await.expect("should be working");
        }
        Ok(())        
    }
}
