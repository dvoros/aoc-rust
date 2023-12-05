fn main() {
    let mut parts = include_str!("../input").split("\n\n");
    let seeds: Vec<(usize, usize)> = parts
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    
    let conversions: Vec<_> = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let nums = line.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (nums[0], nums[1], nums[2])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut inputs = seeds;
    let mut results = vec![];
    for round in conversions {
        results = vec![];
        for rule in round.iter() {
            let mut remainders = vec![];
            for input in inputs.iter() {
                let (overlap, mut remainder) = convert(*input, &rule);
                if let Some(overlap) = overlap {
                    results.push(overlap);
                }
                remainders.append(&mut remainder);
            }
            inputs = remainders;
        }
        inputs.append(&mut results);
    }

    let res = inputs.iter().map(|i| i.0).min().unwrap();
    println!("result: {res}");
}

fn convert(input: (usize, usize), conversion: &(usize, usize, usize)) -> (Option<(usize, usize)>, Vec<(usize, usize)>) {
    let overlap_start = input.0.max(conversion.1);
    let overlap_end = (input.0 + input.1).min(conversion.1 + conversion.2);
    if overlap_start >= overlap_end {
        return (None, vec![input]);
    }

    let mut remaining = vec![];
    let overlap = (overlap_start + conversion.0 - conversion.1, overlap_end-overlap_start);
    
    if overlap_start > input.0 {
        remaining.push((input.0, overlap_start-input.0))
    }

    if overlap_end < input.0 + input.1 {
        remaining.push((overlap_end, input.0 + input.1 - overlap_end))
    }

    (Some(overlap), remaining)
}

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn it_converts() {
        //    [ () ]
        let (overlap, rem) = convert((12, 6), &(20, 10, 10));
        assert_eq!(overlap, Some((22, 6)));
        assert_eq!(rem, vec![]);

        //  ()[    ]
        let (overlap, rem) = convert((2, 8), &(20, 10, 10));
        assert_eq!(overlap, None);
        assert_eq!(rem, vec![(2, 8)]);

        //    [    ]()
        let (overlap, rem) = convert((20, 6), &(20, 10, 10));
        assert_eq!(overlap, None);
        assert_eq!(rem, vec![(20, 6)]);

        //  ( [)   ]
        let (overlap, rem) = convert((2, 9), &(20, 10, 10));
        assert_eq!(overlap, Some((20, 1)));
        assert_eq!(rem, vec![(2, 8)]);

        //    [   (] )
        let (overlap, rem) = convert((19, 5), &(20, 10, 10));
        assert_eq!(overlap, Some((29, 1)));
        assert_eq!(rem, vec![(20, 4)]);

        //  ( [    ] )
        let (overlap, rem) = convert((8, 15), &(20, 10, 10));
        assert_eq!(overlap, Some((20, 10)));
        assert_eq!(rem, vec![(8, 2), (20, 3)]);


        let (overlap, rem) = convert((15, 10), &(20, 10, 10));
        assert_eq!(overlap, Some((25, 5)));
        assert_eq!(rem, vec![(20, 5)]);
    }
}