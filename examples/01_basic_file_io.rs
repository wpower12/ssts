use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    open_and_print(path);

    // If I want to append, I need to use the OpenOptions stuff.
    // this lets me set more options, like append/write.
    // Note, examples for this usually include a call to
    // unwrap(), which I think does some syntactic sugar magic
    // to deal with the others returning Results? Not sure.
    // I took it out, and the above code worked fine, that is
    // I could easily add the match statement like we did before.
    let mut append_file = match OpenOptions::new().write(true).append(true).open(path) {
        Err(why)        => panic!("couldn't open {:?} : {}", path, why),
        Ok(opened_file) => opened_file,
    };

    // We use the writeln! macro to write a line to a file. Note
    // that to handle the Result type, we match the output
    // of the macro against the two possible 'things' that can
    // be in a type of Result; Err and Ok.
    match writeln!(append_file, "A new line!") {
        Err(why) => panic!("couldn't append. : {}", why),
        Ok(_)    => println!("appended line!\n"),
    }
    drop(append_file);

    open_and_print(path);
}

// Note the syntax for typing parameters; var_name: <TYPE>
// you are saying "path is of type 'BorrowedRef to a Path'"
fn open_and_print(path: &Path) {
    let display = path.display();
    // Look, a match statement! We use it to match on the thing
    // returned by the open method. If its of type Err, we get
    // that type, with a 'why' variable passed to us. If its of
    // type Ok, we get the file passed to us. The arms handle the
    // two cases. So if its an err, file never gets something, we
    // just panic. If its ok, we assign the Ok(file)'s opened_file
    // to the variable file.
    let mut read_file = match File::open(&path) {
        Err(why)        => panic!("couldn't open {} : {}", display, why),
        Ok(opened_file) => opened_file,
    };

    let mut s = String::new();
    match read_file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file {} : {}", display, why),
        Ok(_)    => print!("{} contains: \n{}\n", display, s),
    };

    // file references are closed automatically when they go out of
    // scope. To force it you use the drop function. I'm not sure I
    // still need this now that the reading is happening in its
    // own function scope. Probably not? 
    drop(read_file);
}
