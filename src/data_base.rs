use std::fs;
use anyhow::{anyhow, Ok, Result};
use crate::models::{DataBaseState, Epic, Story, Status};

pub struct JiraDatabase {
    database: Box<dyn Database>
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self { database: Box::new(JSONFileDatabase { file_path }) }
    }

    pub fn read_db(&self) -> Result<DataBaseState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed = self.database.read_db()?;

        let last_id = parsed.last_item_id;
        let new_id = last_id + 1;

        parsed.last_item_id = new_id;
        parsed.epics.insert(new_id, epic);

        self.database.write_db(&parsed)?;
        Ok(new_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.database.read_db()?;

        let last_id = parsed.last_item_id;
        let new_id = last_id + 1;

        parsed.last_item_id = new_id;
        parsed.stories.insert(new_id, story);
        parsed
            .epics.get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Could not find epic in database!"))?
            .stories.push(new_id);

        self.database.write_db(&parsed)?;
        Ok(new_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut parsed = self.database.read_db()?;

        for story_id in &parsed.epics
                .get(&epic_id).ok_or_else(|| anyhow!("Could not find epic in database!"))?
                .stories {
            parsed.stories.remove(&epic_id);
        }

        parsed.epics.remove(&epic_id);

        self.database.write_db(&parsed)?;
        Ok(())
    }
}

trait Database {
    fn read_db(&self) -> Result<DataBaseState>;
    fn write_db(&self, db_state: &DataBaseState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DataBaseState> {
        let database_content = fs::read_to_string(&self.file_path)?;
        let parsed: DataBaseState = serde_json::from_str(&database_content)?;
        Ok(parsed)
    }

    fn write_db(&self, database_state: &DataBaseState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(database_state)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let database = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(database.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmp_file, "{}", file_contents).unwrap();

            let database = JSONFileDatabase { file_path: tmp_file.path().to_str()
                        .expect("Failed to convert tmp_file path to str").to_string() };
            let result = database.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmp_file, "{}", file_contents).unwrap();

            let database = JSONFileDatabase { file_path: tmp_file.path().to_str()
                        .expect("Failed to convert tmp_file to str").to_string() };
            let result = database.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_to_db_should_work() {
            let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmp_file, "{}", file_contents).unwrap();

            let database = JSONFileDatabase { file_path: tmp_file.path().to_str()
                        .expect("Failed to convert tmp_file path to str").to_string() };
            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };
            
            let mut stories = HashMap::new();
            stories.insert(2, story);
            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DataBaseState { last_item_id: 2, epics, stories };

            let write_result = database.write_db(&state);
            let read_result = database.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}