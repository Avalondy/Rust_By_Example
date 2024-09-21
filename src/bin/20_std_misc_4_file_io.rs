mod open {
    use std::{fs::File, io::prelude::*, path::Path};

    pub fn test() {
        // Create a path to the desired file
        let path = Path::new("../../data/hello.txt");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open {}: {}", display, why),
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => print!("{} contains:\n{}", display, s),
            Err(why) => panic!("couldn't read {}: {}", display, why),
        }

        // `file` goes out of scope, and the "hello.txt" file gets closed
    }
}

mod create {
    use std::{fs::File, io::prelude::*, path::Path};

    static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

    pub fn test() {
        let path = Path::new("../../data/lorem_ipsum.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't create {}: {}", display, why),
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Ok(_) => println!("successfully wrote to {}", display),
            Err(why) => panic!("couldn't write to {}: {}", display, why),
        };
    }
}

mod read_lines {
    use std::{
        fs::File,
        io::{self, BufRead},
        path::Path,
    };

    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn test() {
        if let Ok(lines) = read_lines("../../data/hosts.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.flatten() {
                println!("{line}");
            }
        }
    }
}

fn main() {
    open::test();
    create::test();
    read_lines::test();
}
