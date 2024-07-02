use crate::{user_interface::{Page, HomePage, EpicDetail, StoryDetail, Prompts}, data_base::JiraDatabase, models::Action};
use std::rc::Rc;

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    db: Rc<JiraDatabase>
}

