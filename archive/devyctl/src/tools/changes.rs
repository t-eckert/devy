use lib::Git;

pub fn changes() {
    let version = std::fs::read_to_string("./version").expect("Could not read version file");

    let git = Git::new("/opt/homebrew/bin/git").expect("Could not find git");
    let hist = git
        .hist(".", &version, "HEAD")
        .expect("Could not get git history");

    println!("{}", hist);
}
