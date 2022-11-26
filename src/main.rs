use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use cursive::Cursive;
use cursive::view::Nameable;
use cursive::view::Scrollable;
use cursive::views::Dialog;
use cursive_tree_view::{Placement, TreeView};
use cursive_table_view::{TableView, TableViewItem};

// walkdir = "2" is a potential crate to traverse directories

#[derive(Debug)]
struct TreeEntry {
    name: String,
    dir: Option<PathBuf>,
}

impl fmt::Display for TreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }

}

fn main() {



    let mut prg = cursive::default();
    
    let dir: PathBuf = get_current_dir();


    prg.add_layer(Dialog::text("Git Tracker")
        .title(dir.as_path().display().to_string())
        .button("Next", sec_view));

    prg.run();



}

fn sec_view(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Do you want to track all your git repos?")
        .title("Track repos?")
        .button("Yes!", track_view)
        .button("Not Now.", |s| quit_view(s, "Are you sure you want to quit?")));
}

// fn track_view(s: &mut Cursive) {
//     s.pop_layer();
//     s.add_layer(Dialog::text("Look at those!")
//         .title("All those git files")
//         .button("Exit", |s| quit_view(s, "Are you sure you want to quit?")));
//
// }

// add tree view

fn track_view(s: &mut Cursive) {

    let mut tree = TreeView::<TreeEntry>::new();
    let path = env::current_dir().expect("Working directory missing.");

    tree.insert_item(
        TreeEntry {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            dir: Some(path.clone()),
        },
        Placement::After,
        0,
        );

    expand_tree(&mut tree, 0, &path);

    // Lazily insert directory listings for sub nodes
    tree.set_on_collapse(|siv: &mut Cursive, row, is_collapsed, children| {
        if !is_collapsed && children == 0 {
            siv.call_on_name("tree", move |tree: &mut TreeView<TreeEntry>| {
                if let Some(dir) = tree.borrow_item(row).unwrap().dir.clone() {
                    expand_tree(tree, row, &dir);
                }
            });
        }
    });
    s.pop_layer();
    s.add_layer(Dialog::around(tree.with_name("tree").scrollable()).title("File View"));
    
}


fn quit_view(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Quit")
        .button("Yes", |s| s.quit())
        .button("No", track_view));
}

fn collect_entries(dir: &PathBuf, entries: &mut Vec<TreeEntry>) -> io::Result<()> {
    if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    entries.push(TreeEntry {
                        name: entry
                            .file_name()
                            .into_string()
                            .unwrap_or_else(|_| "".to_string()),
                        dir: Some(path),
                    });
                } else if path.is_file() {
                    entries.push(TreeEntry {
                        name: entry
                            .file_name()
                            .into_string()
                            .unwrap_or_else(|_| "".to_string()),
                        dir: None,
                    });
                }
            }
        }
        Ok(())
}

fn expand_tree(tree: &mut TreeView<TreeEntry>, parent_row: usize, dir: &PathBuf) {
    let mut entries = Vec::new();
        collect_entries(dir, &mut entries).ok();

        entries.sort_by(|a, b| match (a.dir.is_some(), b.dir.is_some()) {
            (true, true) | (false, false) => a.name.cmp(&b.name),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
        });

        for i in entries {
            if i.dir.is_some() {
                tree.insert_container_item(i, Placement::LastChild, parent_row);
            } else {
                tree.insert_item(i, Placement::LastChild, parent_row);
            }
        }
}

//add function here to read fs path and outpath cur path
fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}
