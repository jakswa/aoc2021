fn main() -> std::io::Result<()> {
    let mut yvel: i64 = 0;
    let mut xy: (i64, i64) = (0,0);

    std::fs::read_to_string("input.txt")?.lines().map(|i| {
        let mut line = i.split(" ");
        (
            line.next().expect("um"),
            line.next().expect("um").parse::<i64>().expect("um")
        )
    }).for_each(|i| {
        match i.0 {
            "down" => yvel += i.1,
            "up" => yvel -= i.1,
            "forward" => {
                xy.1 += i.1;
                xy.0 += yvel * i.1;
            },
            _ => {}
        }
        ()
    });
    println!("WEE: X{},Y{}, {}", xy.0, xy.1, xy.0 * xy.1);
    Ok(())
}
