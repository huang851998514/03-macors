use anyhow::Result;
use onion_rust_template::my_vec;

fn main() -> Result<()> {
    let v:Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];
    println!("{:?}", v);
    Ok(())
}
