use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RingMember {
    pub title: String,
    pub author: String,
    pub url: String,
}
pub struct Ring {
    pub members: HashMap<String, RingMember>,
    ordered_ids: Vec<String>,
}

impl Ring {
    pub fn from_members(members: HashMap<String, RingMember>) -> Self {
        return Ring {
            ordered_ids: members.keys().cloned().collect(),
            members: members,
        };
    }
    pub fn get_index_for_id(&self, id: String) -> Option<usize> {
        return self.ordered_ids.iter().position(|r| r == &id).into();
    }
    pub fn next_id(&self, id: String) -> Option<&RingMember> {
        if let Some(index) = self.get_index_for_id(id) {
            return Some(self.next(index));
        }
        return None;
    }
    pub fn prev_id(&self, id: String) -> Option<&RingMember> {
        if let Some(index) = self.get_index_for_id(id) {
            return Some(self.prev(index));
        }
        return None;
    }
    pub fn next(&self, index: usize) -> &RingMember {
        let len = self.ordered_ids.len();
        let next_id = self.ordered_ids[((index + 1) % len + len) % len].clone();
        return &self.members[&next_id];
    }
    pub fn prev(&self, index: usize) -> &RingMember {
        let len = self.ordered_ids.len();
        let next_id = self.ordered_ids[((index - 1) % len + len) % len].clone();
        return &self.members[&next_id];
    }
}
