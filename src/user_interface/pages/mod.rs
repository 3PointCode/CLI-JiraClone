use std::any::Any;
use std::rc::Rc;
use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;
use crate::data_base::JiraDatabase;
use crate::models::Action;
use itertools::Itertools;

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        let epics = self.db.read_db()?.epics;

        for id in epics.keys().sorted() {
            let epic = &epics[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&epic.name, 32);
            let status_col = get_column_string(&epic.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");
        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epics = self.db.read_db()?.epics;

        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            input => {
                if let Ok(epic_id) = input.parse::<u32>() {
                    if epics.contains_key(&epic_id) {
                        return Ok(Some(Action::NavigateToEpicDetail { epic_id }));
                    }
                }
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("Could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        let id_col = get_column_string(&self.epic_id.to_string(), 5);
        let name_col = get_column_string(&epic.name, 12);
        let desc_col = get_column_string(&epic.description, 27);
        let status_col = get_column_string(&epic.status.to_string(), 13);
        println!("{} | {} | {} | {}", id_col, name_col, desc_col, status_col);
        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &db_state.stories;

        for id in epic.stories.iter().sorted() {
            let story = &stories[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&story.name, 32);
            let status_col = get_column_string(&story.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        
    }

    fn as_any(&self) -> &dyn Any {
        
    }
}