use crate::{data_base::JiraDatabase, models::{Action, Status}, user_interface::{EpicDetail, HomePage, Page, Prompts, StoryDetail}};
use std::rc::Rc;
use anyhow::{anyhow, Result, Context, Ok};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    db: Rc<JiraDatabase>
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        Self { pages: vec![Box::new(HomePage { db: Rc::clone(&db)})], prompts: Prompts::new(), db }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => {
                self.pages.push(Box::new(EpicDetail { epic_id, db: Rc::clone(&self.db) }));
            }
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                self.pages.push(Box::new(StoryDetail { epic_id, story_id, db: Rc::clone(&self.db) }));
            }
            Action::NavigateToPreviousPage => {
                if !self.pages.is_empty() { self.pages.pop(); }
            }
            Action::CreateEpic => {
                let epic = (self.prompts.create_epic)();
                self.db.create_epic(epic).with_context(|| anyhow!("Failed to create Epic!"))?;
            }
            Action::UpdateEpicStatus { epic_id } => {
                let status = (self.prompts.update_status)();
                if let Some(status) = status{
                    self.db.update_epic_status(epic_id, status).with_context(|| anyhow!("Failed to update Epic status!"))?;
                }
            }
            Action::DeleteEpic { epic_id } => {
                if (self.prompts.delete_epic)() {
                    self.db.delete_epic(epic_id).with_context(|| anyhow!("Failed to delete Epic!"))?;

                    if !self.pages.is_empty() {
                        self.pages.pop();
                    }
                }
            }
            Action::CreateStory { epic_id } => {
                let story = (self.prompts.create_story)();
                self.db.create_story(story, epic_id).with_context(|| anyhow!("Failed to create Story!"))?;
            }
            Action::UpdateStoryStatus { story_id } => {
                let status = (self.prompts.update_status)();
                if let Some(status) = status {
                    self.db.update_story_status(story_id, status).with_context(|| anyhow!("Failed to update Story status!"))?;
                }
            }
            Action::DeleteStory { epic_id, story_id } => {
                if (self.prompts.delete_story)() {
                    self.db.delete_story(epic_id, story_id).with_context(|| anyhow!("Failed to delete Story!"))?;

                    if !self.pages.is_empty() {
                        self.pages.pop();
                    }
                }
            }
            Action::Exit => {
                self.pages.clear()
            },
        }

        Ok(())
    }

    fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    fn set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}

#[cfg(test)]
mod tests {
    use crate::{data_base::test_utils::MockDatabase, models::{Epic, Status, Story}};
    use super::*;

    #[test]
    fn should_start_on_home_page() {
        let db = Rc::new(JiraDatabase { database: Box::new(MockDatabase::new()) });
        let nav = Navigator::new(db);

        assert_eq!(nav.get_page_count(), 1);

        let current_page = nav.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();

        assert_eq!(home_page.is_some(), true);
    }
}