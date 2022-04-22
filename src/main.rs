use git2::BranchType;
use git2::ErrorCode;
use git2::Repository;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".")?;
    let branches = repo.branches(None)?;
    let mut branches_to_delete = Vec::new();

    for branch in branches {
        let (branch, branch_type) = branch?;
        if branch_type == BranchType::Remote {
            continue;
        }

        let name = branch.name()?.unwrap();
        let is_gone = match branch.upstream() {
            Ok(_) => false,
            Err(x) => x.code() == ErrorCode::NotFound,
        };

        if is_gone && branch.is_head() {
            eprintln!("WARN: branch {name} is checked out and gone.");
        } else if is_gone {
            branches_to_delete.push(branch);
        }
    }
    if branches_to_delete.len() > 0 {
        println!("\nDo you want to delete the following branches?");
        for b in branches_to_delete.iter() {
            println!("  - {}", b.name()?.unwrap());
        }
        print!("Are you sure? [y/n]: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        if buf == "y\n" {
            for mut b in branches_to_delete {
                let branch_name = b.name()?.unwrap();
                println!("Deleting branch {branch_name}");
                b.delete()?;
            }
        }
    }

    Ok(())
}
