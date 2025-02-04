//! `ls` example
use rustic_core::{Repository, RepositoryOptions, TreeStreamerOptions};
use simplelog::{Config, LevelFilter, SimpleLogger};

fn main() {
    // Display info logs
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    // Open repository
    let repo_opts = RepositoryOptions {
        repository: Some("/tmp/repo".to_string()),
        password: Some("test".to_string()),
        ..Default::default()
    };

    let repo = Repository::new(&repo_opts)
        .unwrap()
        .open()
        .unwrap()
        .to_indexed()
        .unwrap();

    // use latest snapshot without filtering snapshots
    let node = repo.node_from_snapshot_path("latest", |_| true).unwrap();

    // recursively list the snapshot contents using no additional filtering
    let recursive = true;
    let streamer_opts = TreeStreamerOptions::default();
    for item in repo.ls(&node, &streamer_opts, recursive).unwrap() {
        let (path, _) = item.unwrap();
        println!("{path:?} ");
    }
}
