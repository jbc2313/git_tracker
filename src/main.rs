use cursive::Cursive;
use cursive::views::Dialog;



fn main() {
    let mut prg = cursive::default();

    prg.add_layer(Dialog::text("Git Tracker")
        .title("Tracker Git")
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

fn track_view(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Look at those!")
        .title("All those git files")
        .button("Exit", |s| quit_view(s, "Are you sure you want to quit?")));

}

fn quit_view(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Quit")
        .button("Yes", |s| s.quit()));
}
