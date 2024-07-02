use crate::{user_interface::{Page, HomePage, EpicDetail, StoryDetail, Prompts}, data_base::JiraDatabase, models::Action};
use std::rc::Rc;

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

    fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    fn set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}