// 126. Word Ladder II

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut result = vec![];
        let dict: HashSet<_> = word_list.iter().collect();
        let mut q = VecDeque::from([&begin_word]);
        let mut seen = HashMap::new();
        seen.insert(String::from(&begin_word), 0);
        while !q.is_empty() {
            for _ in 0..q.len() {
                let str = q.pop_front().unwrap();
                let x = seen[str] + 1;
                for i in 0..begin_word.len() {
                    let mut tmp: Vec<_> = str.chars().collect();
                    for c in 'a'..='z' {
                        tmp[i] = c;
                        let tmp: String = tmp.iter().collect();
                        if !seen.contains_key(&tmp) && dict.contains(&tmp) {
                            seen.insert(String::from(&tmp), x);
                            q.push_back(dict.get(&tmp).unwrap());
                        }
                    }
                }
            }
        }
        if seen.contains_key(&end_word) {
            Self::dfs(
                String::from(&end_word),
                &begin_word,
                &mut vec![],
                &mut result,
                &seen,
            );
        }
        result
    }

    fn dfs(
        s: String,
        b: &str,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
        map: &HashMap<String, i32>,
    ) {
        path.push(String::from(&s));
        if b == s {
            let mut shortest_path = path.to_vec();
            shortest_path.reverse();
            result.push(shortest_path);
            path.pop();
            return;
        }
        let current = map[&s];
        for i in 0..s.len() {
            for c in 'a'..='z' {
                let mut tmp: Vec<_> = s.chars().collect();
                tmp[i] = c;
                let tmp: String = tmp.iter().collect();
                if map.contains_key(&tmp) && map[&tmp] == current - 1 {
                    Self::dfs(tmp, b, path, result, map);
                }
            }
        }
        path.pop();
        return;
    }
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            util::str_vec_2_string_vec(vec!["hot", "dot", "dog", "lot", "log", "cog"]),
        );
        assert_eq!(
            vec![
                vec!["hit", "hot", "dot", "dog", "cog"],
                vec!["hit", "hot", "lot", "log", "cog"]
            ],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            util::str_vec_2_string_vec(vec!["hot", "dot", "dog", "lot", "log"]),
        );
        assert_eq!(Vec::<Vec<String>>::new(), result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_ladders(
            "aaaaa".to_string(),
            "ggggg".to_string(),
            util::str_vec_2_string_vec(vec![
                "aaaaa", "caaaa", "cbaaa", "daaaa", "dbaaa", "eaaaa", "ebaaa", "faaaa", "fbaaa",
                "gaaaa", "gbaaa", "haaaa", "hbaaa", "iaaaa", "ibaaa", "jaaaa", "jbaaa", "kaaaa",
                "kbaaa", "laaaa", "lbaaa", "maaaa", "mbaaa", "naaaa", "nbaaa", "oaaaa", "obaaa",
                "paaaa", "pbaaa", "bbaaa", "bbcaa", "bbcba", "bbdaa", "bbdba", "bbeaa", "bbeba",
                "bbfaa", "bbfba", "bbgaa", "bbgba", "bbhaa", "bbhba", "bbiaa", "bbiba", "bbjaa",
                "bbjba", "bbkaa", "bbkba", "bblaa", "bblba", "bbmaa", "bbmba", "bbnaa", "bbnba",
                "bboaa", "bboba", "bbpaa", "bbpba", "bbbba", "abbba", "acbba", "dbbba", "dcbba",
                "ebbba", "ecbba", "fbbba", "fcbba", "gbbba", "gcbba", "hbbba", "hcbba", "ibbba",
                "icbba", "jbbba", "jcbba", "kbbba", "kcbba", "lbbba", "lcbba", "mbbba", "mcbba",
                "nbbba", "ncbba", "obbba", "ocbba", "pbbba", "pcbba", "ccbba", "ccaba", "ccaca",
                "ccdba", "ccdca", "cceba", "cceca", "ccfba", "ccfca", "ccgba", "ccgca", "cchba",
                "cchca", "cciba", "ccica", "ccjba", "ccjca", "cckba", "cckca", "cclba", "cclca",
                "ccmba", "ccmca", "ccnba", "ccnca", "ccoba", "ccoca", "ccpba", "ccpca", "cccca",
                "accca", "adcca", "bccca", "bdcca", "eccca", "edcca", "fccca", "fdcca", "gccca",
                "gdcca", "hccca", "hdcca", "iccca", "idcca", "jccca", "jdcca", "kccca", "kdcca",
                "lccca", "ldcca", "mccca", "mdcca", "nccca", "ndcca", "occca", "odcca", "pccca",
                "pdcca", "ddcca", "ddaca", "ddada", "ddbca", "ddbda", "ddeca", "ddeda", "ddfca",
                "ddfda", "ddgca", "ddgda", "ddhca", "ddhda", "ddica", "ddida", "ddjca", "ddjda",
                "ddkca", "ddkda", "ddlca", "ddlda", "ddmca", "ddmda", "ddnca", "ddnda", "ddoca",
                "ddoda", "ddpca", "ddpda", "dddda", "addda", "aedda", "bddda", "bedda", "cddda",
                "cedda", "fddda", "fedda", "gddda", "gedda", "hddda", "hedda", "iddda", "iedda",
                "jddda", "jedda", "kddda", "kedda", "lddda", "ledda", "mddda", "medda", "nddda",
                "nedda", "oddda", "oedda", "pddda", "pedda", "eedda", "eeada", "eeaea", "eebda",
                "eebea", "eecda", "eecea", "eefda", "eefea", "eegda", "eegea", "eehda", "eehea",
                "eeida", "eeiea", "eejda", "eejea", "eekda", "eekea", "eelda", "eelea", "eemda",
                "eemea", "eenda", "eenea", "eeoda", "eeoea", "eepda", "eepea", "eeeea", "ggggg",
                "agggg", "ahggg", "bgggg", "bhggg", "cgggg", "chggg", "dgggg", "dhggg", "egggg",
                "ehggg", "fgggg", "fhggg", "igggg", "ihggg", "jgggg", "jhggg", "kgggg", "khggg",
                "lgggg", "lhggg", "mgggg", "mhggg", "ngggg", "nhggg", "ogggg", "ohggg", "pgggg",
                "phggg", "hhggg", "hhagg", "hhahg", "hhbgg", "hhbhg", "hhcgg", "hhchg", "hhdgg",
                "hhdhg", "hhegg", "hhehg", "hhfgg", "hhfhg", "hhigg", "hhihg", "hhjgg", "hhjhg",
                "hhkgg", "hhkhg", "hhlgg", "hhlhg", "hhmgg", "hhmhg", "hhngg", "hhnhg", "hhogg",
                "hhohg", "hhpgg", "hhphg", "hhhhg", "ahhhg", "aihhg", "bhhhg", "bihhg", "chhhg",
                "cihhg", "dhhhg", "dihhg", "ehhhg", "eihhg", "fhhhg", "fihhg", "ghhhg", "gihhg",
                "jhhhg", "jihhg", "khhhg", "kihhg", "lhhhg", "lihhg", "mhhhg", "mihhg", "nhhhg",
                "nihhg", "ohhhg", "oihhg", "phhhg", "pihhg", "iihhg", "iiahg", "iiaig", "iibhg",
                "iibig", "iichg", "iicig", "iidhg", "iidig", "iiehg", "iieig", "iifhg", "iifig",
                "iighg", "iigig", "iijhg", "iijig", "iikhg", "iikig", "iilhg", "iilig", "iimhg",
                "iimig", "iinhg", "iinig", "iiohg", "iioig", "iiphg", "iipig", "iiiig", "aiiig",
                "ajiig", "biiig", "bjiig", "ciiig", "cjiig", "diiig", "djiig", "eiiig", "ejiig",
                "fiiig", "fjiig", "giiig", "gjiig", "hiiig", "hjiig", "kiiig", "kjiig", "liiig",
                "ljiig", "miiig", "mjiig", "niiig", "njiig", "oiiig", "ojiig", "piiig", "pjiig",
                "jjiig", "jjaig", "jjajg", "jjbig", "jjbjg", "jjcig", "jjcjg", "jjdig", "jjdjg",
                "jjeig", "jjejg", "jjfig", "jjfjg", "jjgig", "jjgjg", "jjhig", "jjhjg", "jjkig",
                "jjkjg", "jjlig", "jjljg", "jjmig", "jjmjg", "jjnig", "jjnjg", "jjoig", "jjojg",
                "jjpig", "jjpjg", "jjjjg", "ajjjg", "akjjg", "bjjjg", "bkjjg", "cjjjg", "ckjjg",
                "djjjg", "dkjjg", "ejjjg", "ekjjg", "fjjjg", "fkjjg", "gjjjg", "gkjjg", "hjjjg",
                "hkjjg", "ijjjg", "ikjjg", "ljjjg", "lkjjg", "mjjjg", "mkjjg", "njjjg", "nkjjg",
                "ojjjg", "okjjg", "pjjjg", "pkjjg", "kkjjg", "kkajg", "kkakg", "kkbjg", "kkbkg",
                "kkcjg", "kkckg", "kkdjg", "kkdkg", "kkejg", "kkekg", "kkfjg", "kkfkg", "kkgjg",
                "kkgkg", "kkhjg", "kkhkg", "kkijg", "kkikg", "kkljg", "kklkg", "kkmjg", "kkmkg",
                "kknjg", "kknkg", "kkojg", "kkokg", "kkpjg", "kkpkg", "kkkkg", "ggggx", "gggxx",
                "ggxxx", "gxxxx", "xxxxx", "xxxxy", "xxxyy", "xxyyy", "xyyyy", "yyyyy", "yyyyw",
                "yyyww", "yywww", "ywwww", "wwwww", "wwvww", "wvvww", "vvvww", "vvvwz", "avvwz",
                "aavwz", "aaawz", "aaaaz",
            ]),
        );
        assert_eq!(
            vec![vec![
                "aaaaa", "aaaaz", "aaawz", "aavwz", "avvwz", "vvvwz", "vvvww", "wvvww", "wwvww",
                "wwwww", "ywwww", "yywww", "yyyww", "yyyyw", "yyyyy", "xyyyy", "xxyyy", "xxxyy",
                "xxxxy", "xxxxx", "gxxxx", "ggxxx", "gggxx", "ggggx", "ggggg"
            ]],
            result
        )
    }
}
