use serde::{Deserialize, Serialize};
use crate::task::Task;

use termion::{color, style};


#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    /// The Shortcut application url for the Story.
    app_url: String,
    /// True if the story has been archived or not.
    archived: bool,
    /// A true/false boolean indicating if the Story is currently blocked.
    blocked: bool,
    /// A true/false boolean indicating if the Story is currently a blocker of another story.   
    blocker: bool,

    // TODO Branches struct
    // branches: Vec<Branch>

    // TODO Comments struct
    // comments: Vec<StoryComment>

    /// A true/false boolean indicating if the Story has been completed.
    completed: bool,
    /// The time/date the Story was completed.
    completed_at: Option<String>,
    /// A manual override for the time/date the Story was completed.
    completed_at_override: Option<String>,
    /// The time/date the Story was created.
    created_at: String,

    // TODO StoryCustomField struct
    // custom_fields: Vec<StoryCustomField>

    /// The cycle time (in seconds) of this story when complete.
    cycle_time: Option<usize>,
    /// The due date of the story.
    deadline: Option<String>,
    /// The description of the story.
    description: String,
    /// A string description of this resource.
    entity_type: String,
    /// The ID of the epic the story belongs to.
    epic_id: Option<usize>,
    /// The numeric point estimate of the story. Can also be null, which means unestimated.
    estimate: Option<usize>,
    /// This field can be set to another unique ID. In the case that the Story has been imported from another tool, the ID in the other tool can be indicated here.
    external_id: Option<String>,
    /// An array of external links (strings) associated with a Story.
    external_links: Vec<String>,

    // TODO UploadedFile struct
    // files Vec<UploadedFile>

    /// An array of UUIDs for any Members listed as Followers.
    follower_ids: Vec<String>,
    /// The ID of the group associated with the story.
    group_id: String,
    /// An array of Group IDs that have been mentioned in the Story description.
    group_mention_ids: Vec<String>,
    /// The unique ID of the Story.
    id: usize,
    /// The ID of the iteration the story belongs to.
    iteration_id: Option<usize>,
    /// An array of label ids attached to the story.
    label_ids: Vec<usize>,

    // TODO LabelSlim struct
    // labels: Vec<LabelSlim>

    lead_time: Option<usize>,
    /// An array of Member IDs that have been mentioned in the Story description.
    member_mention_ids: Vec<String>,
    /// The time/date the Story was last changed workflow-state.
    moved_at: Option<String>,
    /// The name of the story.
    name: String,
    /// A number representing the position of the story in relation to every other story in the current project.
    position: usize,
    /// The IDs of the iteration the story belongs to.
    previous_iteration_ids: Vec<usize>,
    /// The ID of the project the story belongs to.
    project_id: Option<usize>,

    // TODO PullRequest struct
    // pull_requests: Vec<PullRequest>

    /// The ID of the Member that requested the story.
    requested_by_id: String,
    /// A true/false boolean indicating if the Story has been started.
    started: bool,
    /// The time/date the Story was started.
    started_at: Option<String>,
    /// A manual override for the time/date the Story was started.
    started_at_override: Option<String>,
    /// The ID of the story template used to create this story, or null if not created using a template.
    story_template_id: Option<String>,
    /// The type of story (feature, bug, chore).
    story_type: String,
    /// An array of tasks connected to the story.
    tasks: Vec<Task>,

    /// The time/date the Story was updated.
    updated_at: Option<String>,
    /// The ID of the workflow the story belongs to.
    workflow_id: usize,
    /// The ID of the workflow state the story is currently in.
    workflow_state_id: usize,
}

impl Story {
    pub fn print_line(&self) {
        println!(
            "{}{}#{}{}{}: {}",
            style::Bold,
            color::Fg(color::Green),
            self.id,
            style::Reset,
            color::Fg(color::Reset),
            self.name
        );
    }
    pub fn print_tasklist(&self){
        for task in &self.tasks {
            task.print_checkbox_line();
        }
    }
}
