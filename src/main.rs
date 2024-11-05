use colored::*;
use git2::BranchType;
use git2::ErrorCode;
use git2::Repository;
use std::env;
use std::error::Error;
use std::io;
use std::io::Write;

use std::path::PathBuf;
use std::process::Command;

struct LightweightBranch {
    name: String,
    refname: String,
}

fn find_git_repository() -> Option<PathBuf> {
    let mut path = env::current_dir().ok()?;
    loop {
        if path.join(".git").exists() {
            return Some(path);
        }
        path = path.parent()?.to_owned();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = find_git_repository().unwrap_or_else(|| {
        eprintln!(
            "{warn} {end}",
            warn = "WARN:".bold().yellow(),
            end = "No git repository found.".yellow()
        );
        std::process::exit(1);
    });
    let repo = Repository::open(path)?;
    let branches = repo.branches(None)?;
    let mut branches_to_delete = Vec::new();
    let mut has_warned = false;
    let conf = repo.config()?;
    for branch in branches {
        let (branch, branch_type) = branch?;
        if branch_type == BranchType::Remote {
            continue;
        }

        let name = branch.name()?.unwrap().to_string();
        // A _gone_ branch is a branch that has an upstream but where the
        // upstream branch can't be found since it has been deleted.
        let branch_remote_name = format!("branch.{name}.remote");
        let has_upstream = conf.get_string(&branch_remote_name).is_ok();
        let is_gone = has_upstream
            && match branch.upstream() {
                Ok(_) => false,
                Err(x) => x.code() == ErrorCode::NotFound,
            };
        if is_gone && branch.is_head() {
            eprintln!(
                "{warn} {branch} {name} {end}",
                warn = "WARN:".bold().yellow(),
                branch = "branch".yellow(),
                name = name.blue().bold().italic(),
                end = "is gone from upstream, but is currently checked out.".yellow()
            );
            has_warned = true;
        } else if is_gone {
            let oid = branch.into_reference().target().expect("No git ref");
            let refname = format!("{oid}");
            let refname = &refname[0..7];
            branches_to_delete.push(LightweightBranch {
                name,
                refname: refname.to_string(),
            });
        }
    }
    if branches_to_delete.len() > 0 {
        if has_warned {
            println!();
        }
        println!("Do you want to delete the following branches?");
        for b in branches_to_delete.iter() {
            println!("  - {} ({})", b.name, b.refname);
        }
        print!("Are you sure? [y/n]: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        if buf == "y\n" {
            delete_branch(
                &branches_to_delete
                    .iter()
                    .map(|b| b.name.clone())
                    .collect::<Vec<String>>(),
            );
        }
    }

    Ok(())
}

fn delete_branch(names: &[String]) {
    // Using GitBranch.delete() didn't delete everything. It left the
    // [branch "master"] lines in the git config.
    // It's also nice to use proper git. Gives nice output.
    let mut args = vec!["branch", "-D"];
    for name in names {
        args.push(name)
    }
    Command::new("git")
        .args(args)
        .spawn()
        .expect("Failed to spawn")
        .wait()
        .expect("Failed to wait");
}
