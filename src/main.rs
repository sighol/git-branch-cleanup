use git2::Repository;
fn main() {
    let repo = Repository::open(".").unwrap();
    let branches = repo.branches(None).unwrap();
    for b in branches {
        let (branch, branch_type) = b.unwrap();
        println!("{}", branch.name().unwrap().unwrap())
    }
}
