use clap::Parser;

use std::error::Error;
use flactags::cli::Cli;
use flactags::utils::{ Tag, TagEditor, InteractiveTagEditor };
use flactags::constants::{
    TITLE,
    ARTIST,
    ALBUM,
    GENRE,
    TRACK_NUMBER
};


fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();

    let directory = &args.dir.unwrap_or(
        std::env::current_dir()?
    );

    let mut editor = TagEditor::new(directory)?;
    

    if args.artist.is_some() {
        editor.set_tags(
            ARTIST,
            &args.artist.unwrap()
        )
    }

    if args.album.is_some() {
        editor.set_tags(
            ALBUM,
            &args.album.unwrap()
        )
    }

    if args.genre.is_some() {
        editor.set_tags(
            GENRE,
            &args.genre.unwrap()
        )
    }

    if args.print {
        editor.print_tags();
    }

    let mut interactive = InteractiveTagEditor::new(&mut editor);

    if args.titles {
        interactive.edit_tags(TITLE)?;
    }

    if args.track_numbers {
        interactive.edit_tags(TRACK_NUMBER)?;
    }

    editor.save()?;

    Ok(())
}