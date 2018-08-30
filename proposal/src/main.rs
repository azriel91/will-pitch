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
    t.fg(203).unwrap();
    print!("{}", style(" people's free will and ").bold());
    print_sub(&mut t, "force");
    t.fg(203).unwrap();
    print!("{}", style(" them ").bold());
    print_sub(&mut t, "to learn");
    t.fg(203).unwrap();
    println!("{}", style(" technical skills.").bold());
    t.reset().unwrap();

    print_add(&mut t, "Drive");
    t.fg(155).unwrap();
    print!("{}", style(" people's free will ").bold());
    print_add(&mut t, "by providing");
    t.fg(155).unwrap();
    print!("{}", style(" them ").bold());
    print_add(&mut t, "with a game that teaches them");
    t.fg(155).unwrap();
    print!("{}", style(" technical skills").bold());
    print_add(&mut t, " while playing");
    t.fg(155).unwrap();
    println!("{}", style(".").bold());
    t.reset().unwrap();
}

fn print_add(t: &mut Box<StdoutTerminal>, text: &str) {
    t.bg(color::GREEN).unwrap();
    t.fg(155).unwrap();
    write!(t, "{}", style(text).bold()).unwrap();
    t.reset().unwrap();
}

fn print_sub(t: &mut Box<StdoutTerminal>, text: &str) {
    t.bg(color::RED).unwrap();
    t.fg(203).unwrap();
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
