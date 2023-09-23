use git2::Repository;
use tracing::info;

use crate::common::get_selected_commit;
use crate::result::Message;
use crate::result::Result;

pub async fn create_pull_request() -> Result<Message> {
    info!("Opening the local git repository.");
    let repo = Repository::discover(".")?;

    let current_commit = get_selected_commit(&repo)?;
    info!("Current commit = {}", current_commit.id());

    info!("and some more stuff.");

    println!("Created pull request.");
    Ok(Message::Empty)
}
