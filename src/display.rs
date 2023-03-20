use colored::Colorize;

fn display_todo(msg: &mut String) {
    msg.replace_range(..5, "");
    let body = msg.trim();
    println!(
        "{} {}",
        "TODO:".bold().blue().underline(),
	body)
}

pub fn display_todos(msgs: Vec<String>) {
    for mut msg in msgs {
	display_todo(&mut msg);
    }
}
