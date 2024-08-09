use eyre::eyre;

pub fn build(left: u64, right: u64) -> eyre::Result<()> {

    println!("left: {}, right: {}", left, right);
    let cmd = "running cmd";
    Err(eyre!("very very bad things have happened, {}", cmd))
    
}

pub fn build_extended(left: u64, right: u64, middle: String) -> eyre::Result<()> {
    println!("left: {}, middle: {}, right: {}", left, middle, right);
    let cmd = "running even worse cmd";
    Err(eyre!("even worse things have happened, {}", cmd))
}

