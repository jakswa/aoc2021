fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    //let input = std::fs::read_to_string("example.txt")?;

    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut ind: usize = 0;
    loop {
        let line_count = lines.len();
        if lines.len() == 1 {
            break;
        }
        let zero_count = lines
            .iter()
            .filter(|l| l.chars().nth(ind).unwrap() == '0')
            .count();
        let target = match line_count - zero_count < zero_count {
            true => '0',
            false => '1',
        };

        lines = lines
            .iter()
            .map(|i| *i)
            .filter(|l| {
                let curchar = l.chars().nth(ind).unwrap();
                println!("fuck: {}", curchar);
                curchar == target
            })
            .collect::<Vec<&str>>();

        ind += 1;
    }
    let o2r = isize::from_str_radix(lines[0], 2).unwrap();

    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut ind: usize = 0;
    loop {
        let line_count = lines.len();
        if lines.len() == 1 {
            break;
        }
        let zero_count = lines
            .iter()
            .filter(|l| l.chars().nth(ind).unwrap() == '0')
            .count();
        let target = match line_count - zero_count < zero_count {
            true => '1',
            false => '0',
        };

        lines = lines
            .iter()
            .map(|i| *i)
            .filter(|l| {
                let curchar = l.chars().nth(ind).unwrap();
                println!("fuck: {}", curchar);
                curchar == target
            })
            .collect::<Vec<&str>>();

        ind += 1;
    }
    let other = isize::from_str_radix(lines[0], 2).unwrap();

    println!("final is: {:?}", o2r * other);
    Ok(())
}

