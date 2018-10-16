# hrm
human readable mtime

---

hrm shows the time elapsed since last modification in the format Y(ears) D(ays) H(ours) M(inutes) S(econds). Example:
<pre>
$ hrm Cargo.toml 
FILE:                                             AGE:
Cargo.toml                                        Y  D   H  M  S  
-----------------------------------------------------------------
                                                  0  1   23 16 56 
</pre>
Written to complement both `ls -l` and `stat`, both shows the actual _time_ of the last modification, but not the _elapsed_ time.

# Documentation

```
hrm 0.1.0
Magnus Wallin <magnuswallin@tutanota.com>

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
    order as default. If you want descending order, use the -d flag.
    Directories are denoted by a '/' at the end of the name.

USAGE:
    hrm [FLAGS] <TARGET>

FLAGS:
    -d, --sort_desc    Sort output from directory in descending order.
    -h, --help         Prints help information
    -V, --version      Prints version information

ARGS:
    <TARGET>    Target file or directory

```
