

#[derive(thiserror::Error, Debug)]
#[error(" WrapIOError : {msg} on file {path}")]
pub struct WrapIOError {
    msg: String,
    path: String,
    #[source]  // optional if field name is `source`
    source: std::io::Error,
}

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error("TestError: regular `{0}`")]
    Regular(String),
    #[error("TestError: from read file")]
    FromReadFile(#[from] WrapIOError),
}

fn read_file(path: &str) -> Result<String, WrapIOError> {
    std::fs::read_to_string(path).map_err(|io|
        WrapIOError {
            msg: "this occurred very very deep".into(),
            source: io, 
            path: path.into()
        }
    )
}

pub fn build(left: u64, right: u64) -> Result<(), TestError> {

    println!("left: {}, right: {}", left, right);
    let cmd = "running cmd";
    Err(TestError::Regular(format!("very very bad things have happened, {}", cmd)))
    
}

pub fn build_extended(_left: u64, _right: u64, middle: String) -> Result<(), TestError> {
    // println!("left: {}, middle: {}, right: {}", left, middle, right);
    read_file(&middle)?;
    Ok(())
}

