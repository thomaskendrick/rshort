use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct StoryComment {
    /// The text of the Comment. In the case that the Comment has been deleted, this field can be set to nil.  
    text: Option<String>,
}
