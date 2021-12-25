fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cmds = input
        .lines()
        .map(|line| {
            let on = &line[0..2] == "on";
            let ranges = line.split('=').skip(1);
            let ranges = ranges
                .map(|c| {
                    let mut nums = c
                        .split(',')
                        .next()
                        .unwrap()
                        .split("..")
                        .map(|i| i.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    nums.sort();
                    nums
                })
                .collect::<Vec<Vec<i64>>>();
            (on, ranges)
        })
        .collect::<Vec<(bool, Vec<Vec<i64>>)>>();

    let xmin = cmds.iter().map(|(_, ranges)| ranges[0][0]).min().unwrap();
    let xmax = cmds.iter().map(|(_, ranges)| ranges[0][1]).max().unwrap();
    let ymin = cmds.iter().map(|(_, ranges)| ranges[1][0]).min().unwrap();
    let ymax = cmds.iter().map(|(_, ranges)| ranges[1][1]).max().unwrap();
    let zmin = cmds.iter().map(|(_, ranges)| ranges[2][0]).min().unwrap();
    let zmax = cmds.iter().map(|(_, ranges)| ranges[2][1]).max().unwrap();
    let mut on: u64 = 0;
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            for z in zmin..=zmax {
                match cmds.iter().rev().find(|(_onoff, ranges)| {
                    x >= ranges[0][0]
                        && x <= ranges[0][1]
                        && y >= ranges[1][0]
                        && y <= ranges[1][1]
                        && z >= ranges[2][0]
                        && z <= ranges[2][1]
                }) {
                    Some(cmd) => {
                        if cmd.0 {
                            on += 1;
                        }
                    }
                    None => {}
                }
            }
        }
    }
    println!("Hello, world! {:?}", on);
}

