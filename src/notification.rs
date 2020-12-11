use super::action::Action;
use std::collections::HashMap;

pub struct Notification {
    actions: HashMap<String, Action>,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn add_action(&mut self, id: String, action: Action) {
        self.actions.insert(id, action);
    }

    #[allow(unused)]
    pub fn invoke_action(&self, action_id: String) {
        if let Some(action) = self.actions.get(&action_id) {
            action.invoke();
        }
    }

    pub fn actions(&self) -> Vec<&str> {
        let mut actions = vec![];
        for (id, action) in self.actions.iter() {
            actions.push(id as &str);
            actions.push(&action.label as &str);
        }
        actions
    }

    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }
}
