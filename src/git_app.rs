use std::{collections::HashSet, thread, time};

use git2::{Repository, StatusOptions, Statuses};

#[derive(Hash, Eq, PartialEq, Debug)]
enum StagingState{
    Staging,
    Staged
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Staging{
    file_path: String,
    state: StagingState
}
impl Staging{
    fn new(file_path: String, state: StagingState) -> Self{
        Self{
            file_path: file_path,
            state: state
        }
    }
}

#[warn(unused_imports)]
pub fn get_repository() -> Result<(), git2::Error> {
    let mut staging_files: HashSet<Staging> = HashSet::new();
    let repo = Repository::open(".")?;
    if repo.is_bare() {
        return Err(git2::Error::from_str("Erro"));
    }
    let mut opts = StatusOptions::new();
    let statuses = repo.statuses(Some(&mut opts))?;
    //git_status(&mut staging_files, &statuses);

    return Ok(());
}

fn git_status(staging_file: &mut HashSet<Staging>, statuses: &Statuses){
    loop{
        print!("teste");
        for entry in statuses.iter() {
            let file = entry.index_to_workdir().unwrap().old_file().path().unwrap();
            staging_file.insert(Staging::new(file.display().to_string(), StagingState::Staging));
        }
        thread::sleep(time::Duration::from_millis(100))
    }
}
