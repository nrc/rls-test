extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rls_analysis as analysis;

use analysis::{AnalysisHost, Target, Id};

use std::env;
use std::process::Command;

fn main() {
    env_logger::init().unwrap();

    // Build
    let mut cmd = Command::new("cargo");
    cmd.arg("check");
    cmd.env("RUSTFLAGS", "-Zunstable-options -Zsave-analysis");
    cmd.env("CARGO_TARGET_DIR", "target/rls");
    cmd.env("RUST_LOG", "");
    cmd.status().unwrap();

    // Analysis
    let analysis = AnalysisHost::new(Target::Debug);

    let project_dir = env::current_dir().unwrap();
    analysis.reload(&project_dir, &project_dir, true).unwrap();

    let roots = analysis.def_roots().unwrap();
    for (id, name) in roots {
        println!("crate {}: {}", name, id);

        if name != "rls_test" {
            continue;
        }

        print_children("  ", id, &analysis);
    }
}

fn print_children(indent: &str, id: Id, analysis: &AnalysisHost) {
    let mut children = vec![];
    analysis.for_each_child_def(id, &mut |id, def: &::analysis::Def| {
        children.push((id, def.clone()));
    }).unwrap();

    for (id, def) in children {
        println!("{}{}: {}", indent, def.name, id);
        print_children(&format!("  {}", indent), id, analysis);
    }
}

struct Foo;

impl Foo {
    fn bar(&self) {}
}
