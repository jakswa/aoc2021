fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let mut cmds = input
        .lines()
        .enumerate()
        .map(|(lind, line)| {
            let on = &line[0..2] == "on";
            let ranges = line.split('=').skip(1);
            let ranges = ranges
                .map(|c| {
                    let mut nums = c
                        .split(',')
                        .next()
                        .unwrap()
                        .split("..")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    nums.sort();
                    nums
                })
                .collect::<Vec<Vec<i32>>>();
            (
                lind,
                on,
                [
                    ranges[0][0],
                    ranges[0][1],
                    ranges[1][0],
                    ranges[1][1],
                    ranges[2][0],
                    ranges[2][1],
                ],
            )
        })
        .collect::<Vec<(usize, bool, [i32; 6])>>();
}

fn overlaps(s: &[i32; 6], o: &[i32; 6]) -> bool {
    !(s[0] > o[1] || s[1] < o[0] || s[2] > o[3] || s[3] < o[2] || s[4] > o[5] || s[5] < o[4])
}

fn swallows(s: &[i32; 6], o: &[i32; 6]) -> bool {
    s[0] < o[0] && s[1] > o[1] && s[2] < o[2] && s[3] > o[3] && s[4] < o[4] && s[5] > o[5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_point_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[-5, 0, -5, 0, -5, 0]), true);
    }

    #[test]
    fn test_line_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[-5, 0, -5, 0, 0, 5]), true);
    }

    #[test]
    fn test_side_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[-5, 0, 0, 5, 0, 5]), true);
    }

    #[test]
    fn test_basic_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[2, 7, 2, 7, 2, 7]), true);
    }

    #[test]
    fn test_straddling_overlap() {
        assert_eq!(overlaps(&[-1, 7, 2, 4, 2, 4], &[0, 5, 0, 5, 0, 5]), true);
    }

    #[test]
    fn test_stabbing_overlap() {
        assert_eq!(overlaps(&[2, 8, 2, 4, 2, 4], &[0, 5, 0, 5, 0, 5]), true);
    }

    #[test]
    fn test_swallowing_overlap() {
        assert_eq!(overlaps(&[3, 5, 3, 5, 3, 5], &[2, 7, 2, 7, 2, 7]), true);
    }

    #[test]
    fn test_basic_non_overlap() {
        assert_eq!(
            overlaps(&[0, 5, 0, 5, 0, 5], &[12, 17, 12, 17, 12, 17]),
            false
        );
    }
}

