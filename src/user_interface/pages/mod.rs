use std::any::Any;
use std::rc::Rc;
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