use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::path::Path;
//table view imports
use rand::Rng;
use cursive::views::TextView;
use cursive::traits::*;
use cursive::align::HAlign;



//end table view imports
use cursive::Cursive;
use cursive::view::Nameable;
use cursive::view::Scrollable;
use cursive::views::Dialog;
use cursive_tree_view::{Placement, TreeView};
use cursive_table_view::{TableView, TableViewItem};

// walkdir = "2" is a potential crate to traverse directories
// code I made for git repo
#[derive(Debug)]
struct Repo {
    name: String,
    dir: Option<String>,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
//end my code
// code for Tree view
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
// end tree view
// code for table view
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name,
    Dir,
    Rate,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::Dir => "Dir",
            BasicColumn::Rate => "Rate",
        }
    }
}

#[derive(Clone, Debug)]
struct Foo {
    name: String,
    dir: String,
    rate: usize,
}

impl TableViewItem<BasicColumn> for Foo {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Dir => self.dir.to_string(),
            BasicColumn::Rate => format!("{}", self.rate), 
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Dir => self.dir.cmp(&other.dir),
            BasicColumn::Rate => self.rate.cmp(&other.rate),
        }
    }
}



//end table view setup


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
        .button("Not Now.", |s| quit_view(s, "Are you sure you want to quit?"))
        .button("View Table.", table_view));
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

// add logic for table view
fn table_view(s: &mut Cursive) {
   let mut rng = rand::thread_rng();
   let dir = env::current_dir().unwrap();
   let mut repos = Vec::new();
        get_repos(&dir, &mut repos).ok();
   let mut table = TableView::<Foo, BasicColumn>::new()
       .column(BasicColumn::Name, "Name", |c| c.width_percent(20))
        .column(BasicColumn::Dir, "Dir", |c| c.align(HAlign::Center))
        .column(BasicColumn::Rate, "Rate", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });

   let mut items = Vec::new();
   let iter_repos = repos.iter_mut();
    for entry in iter_repos {
        items.push(Foo {
            name: format!("Name {}", entry.name),
            dir: format!("Dir {}", entry.dir.as_ref().unwrap_or(&"NONE".to_owned())),
            rate: rng.gen_range(0..=255),
        });
    }

    table.set_items(items);

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

            siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Removing row # {}", row))
                .button("Close", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        table.remove_item(index);
                    });
                    s.pop_layer();
                }),
        );
    });

    s.add_layer(Dialog::around(table.with_name("table").min_size((50, 20))).title("Table View"));


}

// need to create a new function similar to collect_entries
// the fn will create an iterable list of the entries for the table view
//


fn get_repos(dir: &PathBuf, repos: &mut Vec<Repo>) -> io::Result<()> {
    if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    match path.file_stem() {
                       Some(p) if p == Path::new(".git") => repos.push(Repo {
                            name: String::from("GIT REPO"),
                            dir: Some(path.as_path().display().to_string()),
                        }),
                        _ => repos.push(Repo {
                            name: String::from("NOT GIT"),
                            dir: Some(path.as_path().display().to_string()),
                        })
                    }
                    // repos.push(Repo {
                    //     name: entry
                    //         .file_name()
                    //         .into_string()
                    //         .unwrap_or_else(|_| "".to_string()),
                    //     dir: Some(path.as_path().display().to_string()),
                    // });
                } else if path.is_file() {
                    repos.push(Repo {
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
