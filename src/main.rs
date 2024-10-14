use std::path::Path;
use syntect::dumps::dump_to_file;
use syntect::parsing::SyntaxSet;

fn main() {
    let default_syntax_set = SyntaxSet::load_defaults_newlines();
    let mut builder = default_syntax_set.into_builder();

    // Add all .sublime-syntax files from the "syntaxes" folder
    builder
        .add_from_folder(Path::new("syntaxes"), true)
        .expect("Failed to load syntaxes from folder");

    let syntax_set = builder.build();
    // Serialize the SyntaxSet to a binary file
    dump_to_file(&syntax_set, "syntax_set.bin").expect("Failed to save SyntaxSet");
    println!("Syntax set successfully built and saved to 'syntax_set.bin'");
}
