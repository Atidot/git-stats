#![feature(test)]

use std::collections::HashMap;
use rayon::prelude::*;
use clap::Clap;
use std::path::PathBuf;
use git2::{Repository,Branch,BranchType,Oid,Revwalk};

#[derive(Clap, Debug)]
#[clap(name = "basic")]
struct Opt {
    #[clap(short, long, parse(from_os_str))]
    path: Option<PathBuf>,
}

fn branches(repo: &Repository) -> Vec<(String, String)> {
    repo.branches(None).expect("")
        .filter_map(Result::ok)
        .map(|x| x.0)
        .map(|branch| {
            let name = branch.name().expect("").unwrap().to_string();
            let oid  = branch.into_reference().target().unwrap().to_string();
            (name, oid)
        })
        .collect()
}

fn commits(repo: &Repository, branch_oid: String) -> HashMap<String, (String, i64)> {
    let mut revwalk = repo.revwalk().expect("");
    revwalk.push(Oid::from_str(&branch_oid).expect(""));
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE);
    let authors_ = revwalk.fold(HashMap::new(), |mut hm_inner, rev| {
        let commit = repo.find_commit(rev.expect("")).expect("");
        let id = commit.id().to_string();
        let author = commit.author().to_string();
        let time = commit.time();
        hm_inner.insert(id, (author, time.seconds()));
        hm_inner
    });
    authors_
}

fn walk(repo: &Repository) -> HashMap<String, (String, i64)> {
    let path = &repo.path().to_str().unwrap().to_string();
    let result = branches(&repo)
        .into_par_iter()
        .map(|(branch_name, branch_oid)| {
            let repo = Repository::open(path).expect("");
            commits(&repo, branch_oid)
        })
        .reduce(|| HashMap::new(), |mut acc, hm| {
            acc.extend(hm);
            acc
        });
    result
}

fn walk_serial(repo: &Repository) -> HashMap<String, (String, i64)> {
    let result = branches(&repo)
        .into_iter()
        .map(|(branch_name, branch_oid)| {
            commits(&repo, branch_oid)
        })
        .fold(HashMap::new(), |mut acc, hm| {
            acc.extend(hm);
            acc
        });
    result
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
    let repo = Repository::open(&opt.path.unwrap()).expect("");
    println!("{:#?}", walk(&repo));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    extern crate test;

    //#[bench]
    //fn bench_parallel(b: &mut Bencher) {
        //let repo = Repository::open("/home/barak/Development/atidot/atidot").expect("");
        //b.iter(|| walk(&repo));
    //}

    //#[bench]
    //fn bench_serial(b: &mut Bencher) {
        //let repo = Repository::open("/home/barak/Development/atidot/atidot").expect("");
        //b.iter(|| walk__serial(&repo));
    //}

}
