extern crate console;
extern crate term;

use std::io::prelude::*;

use console::style;
use term::{color, StdoutTerminal};

fn main() {
    println!("{}", style("────────────────────────────────────────────────────────────────────────").bold());
    println!("{}", style("renamed: plan_a to plan_b").bold());
    println!("{}", style("────────────────────────────────────────────────────────────────────────").bold());
    println!("{}", style("@ plan_b:1 @").bold().yellow());

    let mut t = term::stdout().unwrap();

    print_sub(&mut t, "Remove");
    print!(" people's free will and ");
    print_sub(&mut t, "force");
    print!(" them ");
    print_sub(&mut t, "to learn");
    println!(" technical skills.");

    print_add(&mut t, "Drive");
    print!(" people's free will ");
    print_add(&mut t, "by providing");
    print!(" them ");
    print_add(&mut t, "with a game that teaches them");
    print!(" technical skills");
    print_add(&mut t, " while playing");
    println!(".");
}

fn print_add(t: &mut Box<StdoutTerminal>, text: &str) {
    t.fg(color::BRIGHT_GREEN).unwrap();
    t.bg(color::GREEN).unwrap();
    write!(t, "{}", style(text).bold()).unwrap();
    t.reset().unwrap();
}

fn print_sub(t: &mut Box<StdoutTerminal>, text: &str) {
    t.fg(color::BRIGHT_RED).unwrap();
    t.bg(color::RED).unwrap();
    write!(t, "{}", style(text).bold()).unwrap();
    t.reset().unwrap();
}

// fn main() {
//     println!(
//         "{} people's free will and {} them {} technical skills.",
//         style("Remove").red().bold().on_red(),
//         style("force").red().bold().on_red(),
//         style("to learn").red().bold().on_red(),
//     );
//     println!(
//         "{} people's free will and {} them {} technical skills{}.",
//         style("Drive").yellow().bold().on_green(),
//         style("provide").yellow().bold().on_green(),
//         style("with a game that teaches them")
//             .yellow()
//             .bold()
//             .on_green(),
//         style(" while playing").yellow().bold().on_green(),
//     );
// }
