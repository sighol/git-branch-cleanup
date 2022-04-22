use git2::Repository;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".")?;
    let branches = repo.branches(None)?;
    let index = repo.index()?;
    match index.path() {
        Some(path) => println!("path: {:?}", path),
        _ => (),
    }
    for b in branches {
        let (branch, _) = b?;
        let name = branch.name()?.unwrap();
        println!("{} {}", name, if branch.is_head() { "HEAD " } else { "" });
    }

    Ok(())
}
