extern crate unbounded_gpsd;

use unbounded_gpsd::*;

fn main() {
    let mut conn = GpsdConnection::new("127.0.0.1:2947").unwrap();
    conn.watch(true).unwrap();
    loop {
        let resp = conn.get_response();
        match resp {
            Ok(response) => {
                println!("{:?}", response);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
