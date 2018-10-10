extern crate clap;
use clap::{App, Arg};

// Time in seconds
const SEC_PER_MIN: u64 = 60;
const SEC_PER_HOUR: u64 = 60*60;
const SEC_PER_DAY: u64 = SEC_PER_HOUR*24;
const SEC_PER_YEAR: u64 = 31557600;

fn get_mtime(path: &str) -> u64 {
    //* 
    // Get mtime from file or directory
    // Arguments:
    // path as &str
    // Return value:
    // mtime as u64
    //*
    
    // Use this path
    let path = std::path::Path::new(&path);

    // Get metadata
    let file_metadata = std::fs::metadata(path).unwrap();
    let mtime = file_metadata.modified().unwrap().elapsed().unwrap()
        .as_secs();
        
    return mtime;
}

fn mtime_since_now (mtime_in: u64) -> (u64, u64, u64, u64, u64) {
    //*
    // Covert mtime in seconds to years, days, hours, minutes, seconds
    // Arguments:
    // mtime as u64
    // Return value:
    // tuple with years, days, hours, minutes, seconds. All u64)
    //*
    
    let mut mtime_seconds = mtime_in;
    
    let years = mtime_seconds/SEC_PER_YEAR;
    mtime_seconds = mtime_seconds%SEC_PER_YEAR;
    let days = mtime_seconds/SEC_PER_DAY;
    mtime_seconds = mtime_seconds%SEC_PER_DAY;
    let hours = mtime_seconds/SEC_PER_HOUR;
    mtime_seconds = mtime_seconds%SEC_PER_HOUR;
    let minutes = mtime_seconds/SEC_PER_MIN;
    mtime_seconds = mtime_seconds%SEC_PER_MIN;
    let seconds = mtime_seconds;

    return (years, days, hours, minutes, seconds);
}

fn main() {
    static ABOUT: &'static str = "
hrm: human readable mtime

ABOUT:
    Parses mtime into a human readable format in the output of:
    Y(ears) D(ays) H(ours) M(inutes) S(econds).

EXAMPLES:
    1) Run on a file ./foo/zoo
    hrm ./foo/zoo
    FILE:                                             AGE:
    ./foo/zoo                                         Y  D   H  M  S  
    -----------------------------------------------------------------
                                                      0  13  19 46 54 
    
    2) Run on directory ./foo
    hrm ./foo/
    DIR:                                              AGE:
    ./foo/                                            Y  D   H  M  S  
    -----------------------------------------------------------------
    newdir/                                           0  5   0  27 52 
    fooDir/                                           0  12  23 16 11 
    jp                                                0  13  19 48 29
    
NOTES:
    When running on a directory, files/directories are sorted in ascending 
    order as default.
    Directories are denoted by a '/' at the end of the name.
";
    
    // Configure arguments
    let matches = App::new("hrm")
                      .version("0.1.0")
                      .author("Magnus Wallin <magnuswallin@tutanota.com>")
                      .about(ABOUT)
                      .arg(Arg::with_name("TARGET")
                           .help("Target file or directory")
                           .required(true)
                           .index(1))
                      .get_matches();

    let infile = matches.value_of("TARGET").unwrap();

    // Check that we have a valid path
    if std::fs::metadata(&infile).is_err() {
        println!("Error: The path {} is not valid", infile);
        std::process::exit(1);
    }
    
    // If infile is neither dir or file, get out of here!
    if !std::path::Path::new(&infile).is_file() &&  
    !std::path::Path::new(&infile).is_dir() {
        println!("{} is neither file or directory. Leaving.", infile);
        std::process::exit(1);
    }
    
    // Check if we have a file
    if std::path::Path::new(&infile).is_file() {
        let (years, days, hours, minutes, seconds) = 
            mtime_since_now(get_mtime(&infile));
        
        // Print header
        println!("{: <50}{}", "FILE: ", "AGE:");
        println!("{: <50}{: <3}{: <4}{: <3}{: <3}{: <3}",
            infile, "Y", "D", "H", "M", "S");
        println!("-----------------------------------------------------------------");
        
        // Print file age
        println!("{: <50}{: <3}{: <4}{: <3}{: <3}{: <3}",
            "", years, days, hours, minutes, seconds);
    } else if std::path::Path::new(&infile).is_dir() {
        // If we have a directory save filename + mtime in this vec
        let mut file_mtime = Vec::new();
        
        let mut filename;
        
        let dir = std::fs::read_dir(&infile).unwrap();
        for file in dir {
            // Save file object here
            let file_obj = file.unwrap();
            
            filename = file_obj.path().display().to_string();
            
            // Check if we have a directory
            // If so, add a trailing '/' to name
            if std::path::Path::new(&filename).is_dir() {
                let mut dirname = file_obj.path().file_name().unwrap()
                    .to_owned().into_string().unwrap();
                dirname.push_str("/");
                // Add dirname + mtime to vec
                file_mtime.push((dirname, get_mtime(&filename)));
                continue;
            }
            
            // Add filename + mtime to vec
            file_mtime.push((file_obj.path().file_name().unwrap()
                .to_owned().into_string().unwrap(),
                get_mtime(&filename)));
        }
        
        // Exit early if directory is empty
        if file_mtime.len() < 1 {
            println!("Directory {} is empty.", infile);
            std::process::exit(0);
        }
        
        // Sort vector, ascending (default)
        file_mtime.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        
        // Print header
        println!("{: <50}{}", "DIR: ", "AGE:");
        println!("{: <50}{: <3}{: <4}{: <3}{: <3}{: <3}",
        infile, "Y", "D", "H", "M", "S");
        println!("-----------------------------------------------------------------");
        
        // Loop vector
        for tuple in file_mtime {
            let file = tuple.0;
            let (years, days, hours, minutes, seconds) =
                mtime_since_now(tuple.1);
            println!("{: <50}{: <3}{: <4}{: <3}{: <3}{: <3}",
            file, years, days, hours, minutes, seconds);
        }
    }
}
