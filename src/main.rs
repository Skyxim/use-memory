use std::{env, vec};

use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size_str=env::var("APP_SIZE")?;
    let size = parse_size(&size_str);
    println!("allocate {} byte", size);
    let size_100_m = 100 * 1024 * 1024;
    let mut i = 0;
    let mut heap: Vec<Box<Vec<u8>>> = vec![];
    while i < size {
        i += size_100_m;
        let bs = Box::new(vec![1 as u8; size_100_m as usize]);
        heap.push(bs);
    }
    match signal::ctrl_c().await {
        Ok(_) => {
            println!("recevie ctrl^c");
        }
        Err(_) => println!("error recevie signal"),
    };
    _ = heap.len();
    Ok(())
}

fn parse_size(size_str: &str) -> u64 {
    let units = ["KB", "MB", "GB"];
    let mut unit_index: i32 = -1;
    let mut size = 0 as u64;
    for (i, unit) in units.iter().enumerate() {
        if size_str.ends_with(unit) {
            unit_index = i as i32;
            size = (&size_str[0..size_str.len() - 2])
                .parse::<u64>()
                .unwrap_or_default();
        }
    }
    if unit_index < 0 {
        unit_index = 1;
        size = size_str.parse().unwrap_or_default();
    }

    size * ((1024 as u64).pow((unit_index + 1) as u32))
}
