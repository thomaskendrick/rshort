use std::process::Command;

pub fn detect() -> Option<usize> {
    let command = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "--symbolic", "HEAD"])
        .output()
        .unwrap();
        
    let branch = String::from_utf8(command.stdout).unwrap();

    let story_id:  Vec<&str> = branch.split("sc-").collect();

    if story_id.len() > 1 {
        let id: usize = story_id[1].trim().parse().unwrap();
        return Some(id)
    } 
    return None;
}
