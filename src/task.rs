use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// True/false boolean indicating whether the Task has been completed.
    complete: bool,
    /// The time/date the Task was completed.
    completed_at: Option<String>,
    /// The time/date the Task was created.
    created_at: String,
    /// Full text of the Task.
    description: String,
    /// A string description of this resource.
    entity_type: String,
    /// This field can be set to another unique ID. In the case that the Task has been imported from another tool, the ID in the other tool can be indicated here.
    external_id: Option<String>,
    /// An array of UUIDs of Groups mentioned in this Task.
    group_mention_ids: Vec<String>,
    /// The unique ID of the Task.
    id: usize,
    /// An array of UUIDs of Members mentioned in this Task.
    member_mention_ids: Vec<String>,
    /// An array of UUIDs of the Owners of this Task.
    owner_ids: Vec<String>,
    /// The number corresponding to the Task’s position within a list of Tasks on a Story.
    position: usize,
    /// The unique identifier of the parent Story.  
    story_id: usize,
    /// The time/date the Task was updated.
    updated_at: Option<String>,
}

impl Task {
    pub fn print_checkbox_line(&self) {
        let togglebox: &str;
        if self.complete {
            togglebox = "-[x]"
        } else {
            togglebox = "-[ ]"
        }
        println!("\t{} : {}", togglebox, self.description);
    }
}
