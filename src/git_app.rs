use git2::{Error, Repository, Status, StatusOptions};
pub fn get_repository() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    if repo.is_bare() {
        return Err(Error::from_str("Erro"));
    }
    let mut opts = StatusOptions::new();
    let statuses = repo.statuses(Some(&mut opts))?;
    for entry in statuses.iter().filter(|e| e.status() != Status::CURRENT) {
        let istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => "new file: ",
            s if s.contains(git2::Status::INDEX_MODIFIED) => "modified: ",
            s if s.contains(git2::Status::INDEX_DELETED) => "deleted: ",
            s if s.contains(git2::Status::INDEX_RENAMED) => "renamed: ",
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => "typechange:",
            _ => continue,
        };
        println!("{}", istatus);
    }
    return Ok(());
}
