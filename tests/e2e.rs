use std::{error::Error, fs};

use rustaska::{run, Config};
use tempfile::tempdir;

#[test]
fn end_to_end() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir()?;

    let mk_path_txt = |file_name: &str| {
        tmp.path()
            .join(file_name)
            .to_str()
            .expect("Path should be valid")
            .to_owned()
    };

    let src = "example-files/ex1.lst".to_owned();
    let num = mk_path_txt("num.csv");
    let str = mk_path_txt("str.csv");
    let err = mk_path_txt("err.log");

    // some are cloned, they will be needed later
    let args = vec!["0".to_owned(), src, num.clone(), str.clone(), err.clone()].into_iter();

    let cfg = Config::build(args)?;

    run(cfg)?;

    // these should exist, even if empty
    // (panic is better here, it gives a stacktrace
    let msg = "It should exist, even if empty";
    let num_content = fs::read_to_string(num).expect(msg);
    let str_content = fs::read_to_string(str).expect(msg);
    let err_content = fs::read_to_string(err).expect(msg);

    let num_example = fs::read_to_string("example-files/ex1-results/num.csv")?;
    let str_example = fs::read_to_string("example-files/ex1-results/str.csv")?;
    let err_example = fs::read_to_string("example-files/ex1-results/err.txt")?;

    assert_eq!(num_content, num_example);
    assert_eq!(str_content, str_example);
    assert_eq!(err_content, err_example);

    Ok(tmp.close()?)
}
