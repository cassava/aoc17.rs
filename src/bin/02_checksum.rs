extern crate aoc;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 2: {}", PUZZLE);

    let v = input.to_str()
        .lines()
        .map(|s| s.as_ref().unwrap()
                  .split_whitespace()
                  .map(|x| x.parse::<u32>().unwrap())
                  .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    println!(":: Answer 1 is {}", checksum(v.iter()));
    println!(":: Answer 2 is {}", checksum_div(v.iter()));
}

// Calculate the checksum of the spreadsheet.
fn checksum<'a, I>(it: I) -> u32
where I: Iterator<Item = &'a Vec<u32>> {
    it.fold(0, |acc, v| {
        let mut it = v.iter();
        if let Some(x) = it.next() {
            let mut min = x;
            let mut max = x;
            for y in it {
                if y > max {
                    max = y
                }
                if y < min {
                    min = y
                }
            }
            acc + (max - min)
        } else {
            acc
        }
    })
}

// Calculate the checksum of the spreadsheet,
// using the second part of the algorithm.
//
// For each row, the checksum is calculated by
// dividing the largest number that can *cleanly*
// be divided by the smallest number.
//
// Note that this is *not* what is required by the second
// challenge, but it is safer as it makes less assumptions.
fn checksum_div<'a, I>(it: I) -> u32
where I: Iterator<Item = &'a Vec<u32>> {
    it.fold(0, |acc, v| {
        let mut max = 0;
        for x in v.iter() {
            for y in v.iter() {
                if x % y != 0 {
                    continue;
                } else if max < (x / y) {
                    max = x / y;
                }
            }
        }
        acc + max
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spreadsheet_checksum_correct_sum() {
        let tests = vec![
            // vector       max-min  div
            (vec![vec![5,1,9,5]], 8, 9),
            (vec![vec![5,2,9,8]], 7, 4),
            (vec![vec![7,5,3]], 4, 1),
            (vec![vec![5,1,9,5], vec![7,5,3], vec![2,4,6,8]], 18, 14),
        ];

        for t in tests {
            assert_eq!(checksum(t.0.iter()), t.1);
            assert_eq!(checksum_div(t.0.iter()), t.2);
        }
    }
}

const PUZZLE: &'static str = "";
const INPUT: &'static str = r"
798	1976	1866	1862	559	1797	1129	747	85	1108	104	2000	248	131	87	95
201	419	336	65	208	57	74	433	68	360	390	412	355	209	330	135
967	84	492	1425	1502	1324	1268	1113	1259	81	310	1360	773	69	68	290
169	264	107	298	38	149	56	126	276	45	305	403	89	179	394	172
3069	387	2914	2748	1294	1143	3099	152	2867	3082	113	145	2827	2545	134	469
3885	1098	2638	5806	4655	4787	186	4024	2286	5585	5590	215	5336	2738	218	266
661	789	393	159	172	355	820	891	196	831	345	784	65	971	396	234
4095	191	4333	161	3184	193	4830	4153	2070	3759	1207	3222	185	176	2914	4152
131	298	279	304	118	135	300	74	269	96	366	341	139	159	17	149
1155	5131	373	136	103	5168	3424	5126	122	5046	4315	126	236	4668	4595	4959
664	635	588	673	354	656	70	86	211	139	95	40	84	413	618	31
2163	127	957	2500	2370	2344	2224	1432	125	1984	2392	379	2292	98	456	154
271	4026	2960	6444	2896	228	819	676	6612	6987	265	2231	2565	6603	207	6236
91	683	1736	1998	1960	1727	84	1992	1072	1588	1768	74	58	1956	1627	893
3591	1843	3448	1775	3564	2632	1002	3065	77	3579	78	99	1668	98	2963	3553
2155	225	2856	3061	105	204	1269	171	2505	2852	977	1377	181	1856	2952	2262
";
