/*!
# Day 9: Stream Processing

A large stream blocks your path. According to the locals, it's not safe to cross the stream at the
moment because it's full of garbage. You look down at the stream; rather than water, you discover
that it's a stream of characters.

You sit for a while and record part of the stream (your puzzle input). The characters represent
groups - sequences that begin with `{` and end with `}`. Within a group, there are zero or more other
things, separated by commas: either another group or garbage. Since groups can contain other
groups, a `}` only closes the most-recently-opened unclosed group - that is, they are nestable. Your
puzzle input represents a single, large group which itself contains many smaller ones.

Sometimes, instead of a group, you will find garbage. Garbage begins with `<` and ends with `>`.
Between those angle brackets, almost any character can appear, including `{` and `}`. Within garbage,
`<` has no special meaning.

In a futile attempt to clean up the garbage, some program has canceled some of the characters
within it using `!`: inside garbage, any character that comes after `!` should be ignored, including `<`,
`>`, and even another `!`.

You don't see any characters that deviate from these rules. Outside garbage, you only find
well-formed groups, and garbage always terminates according to the rules above.

Here are some self-contained pieces of garbage:

- `<>`, empty garbage.
- `<random characters>`, garbage containing random characters.
- `<<<<>`, because the extra `<` are ignored.
- `<{!>}>`, because the first `>` is canceled.
- `<!!>`, because the second `!` is canceled, allowing the `>` to terminate the garbage.
- `<!!!>>`, because the second `!` and the first `>` are canceled.
- `<{o"i!a,<{i<a>,` which ends at the first `>`.

Here are some examples of whole streams and the number of groups they contain:

- `{}`, 1 group.
- `{{{}}}`, 3 groups.
- `{{},{}}`, also 3 groups.
- `{{{},{},{{}}}}`, 6 groups.
- `{<{},{},{{}}>}`, 1 group (which itself contains garbage).
- `{<a>,<a>,<a>,<a>}`, 1 group.
- `{{<a>},{<a>},{<a>},{<a>}}`, 5 groups.
- `{{<!>},{<!>},{<!>},{<a>}}`, 2 groups (since all but the last `>` are canceled).

Your goal is to find the total score for all groups in your input. Each group is assigned a score
which is one more than the score of the group that immediately contains it. (The outermost group
gets a score of 1.)

- `{}`, score of 1.
- `{{{}}}`, score of 1 + 2 + 3 = 6.
- `{{},{}}`, score of 1 + 2 + 2 = 5.
- `{{{},{},{{}}}}`, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
- `{<a>,<a>,<a>,<a>}`, score of 1.
- `{{<ab>},{<ab>},{<ab>},{<ab>}}`, score of 1 + 2 + 2 + 2 + 2 = 9.
- `{{<!!>},{<!!>},{<!!>},{<!!>}}`, score of 1 + 2 + 2 + 2 + 2 = 9.
- `{{<a!>},{<a!>},{<a!>},{<ab>}}`, score of 1 + 2 = 3.

What is the total score for all groups in your input?
*/

extern crate aoc;

use std::fmt;
use std::error;
use std::str;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 9: {}", PUZZLE);

    let tok: Token = input.to_str().parse().unwrap();
    println!(":: Answer 1 is {}", tok.score());
}

#[derive(Debug,PartialEq)]
pub enum Token {
    Group(Group),
    Garbage(Garbage),
}

impl Token {
    pub fn score(&self) -> usize {
        self._score(1)
    }

    fn _score(&self, depth: usize) -> usize {
        match *self {
            Token::Group(ref g) => g.children.iter().fold(depth, |acc, x| acc + x._score(depth + 1)),
            Token::Garbage(_) => 0,
        }
    }

    fn parse(mut chars: &mut str::CharIndices) -> Result<Self, ParseError> {
        let data = chars.as_str();
        match chars.next() {
            Some((n, '{')) => Group::parse(&mut chars, n).map(|g| Token::Group(g)),
            Some((n, '<')) => Garbage::parse(&mut chars, n).map(|g| Token::Garbage(g)),
            Some((n, _)) => Err(ParseError::with_str(data, n, "unexpected character, expecting one of '{' or '<'")),
            _ => Err(ParseError::with_str(data, data.len(), "unexpected end-of-file")),
        }
    }
}

impl str::FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.char_indices();
        Token::parse(&mut chars)
    }
}

#[derive(Debug,PartialEq)]
pub struct Group {
    data: String,
    children: Vec<Token>,
}

impl Group {
    fn parse(mut chars: &mut str::CharIndices, start: usize) -> Result<Self, ParseError> {
        let mut group = Self{
            data: String::new(),
            children: Vec::new(),
        };
        let data = chars.as_str();
        let mut me = String::from("{");
        while let Some((n, c)) = chars.next() {
            match c {
                '{' => {
                    let tok = Group::parse(&mut chars, n)?;
                    group.children.push(Token::Group(tok));
                }

                '<' => {
                    let tok = Garbage::parse(&mut chars, n)?;
                    group.children.push(Token::Garbage(tok));
                }

                ',' => {}

                '}' => {
                    me.push_str(&data[..(n-start)]); // FIXME!
                    group.data = me;
                    return Ok(group);
                }

                _ => {
                    me.push_str(data);
                    return Err(ParseError::with(me, n, "unexpected character, expecting one of '{', '<', ',', or '}'"));
                }
            }
        }
        me.push_str(data);
        Err(ParseError::with(me, data.len(), "unexpected end-of-file"))
    }
}

#[derive(Debug,PartialEq)]
pub struct Garbage {
    data: String,
}

impl Garbage {
    fn parse(chars: &mut str::CharIndices, start: usize) -> Result<Self, ParseError> {
        let data = chars.as_str();
        let mut me = String::from("<");
        while let Some((n, c)) = chars.next() {
            match c {
                '!' => {
                    if chars.next().is_none() {
                        break;
                    }
                }

                '>' => {
                    me.push_str(&data[..(n-start)]); // FIXME!
                    return Ok(Self{ data: me });
                }

                _ => {}
            }
        }
        me.push_str(data);
        Err(ParseError::with(me, data.len(), "unexpected end-of-file"))
    }
}

#[derive(Debug)]
pub struct ParseError {
    msg: &'static str,
    data: String,
    col: usize,
}

impl ParseError {
    fn new(data: &str, msg: &'static str) -> Self {
        Self {
            msg: msg,
            data: String::from(data),
            col: 0,
        }
    }

    fn with(data: String, col: usize, msg: &'static str) -> Self {
        Self {
            msg: msg,
            data: data,
            col: col,
        }
    }

    fn with_str(data: &str, col: usize, msg: &'static str) -> Self {
        Self {
            msg: msg,
            data: String::from(data),
            col: col,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: at column {} in: {}", self.msg, self.col, self.data)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str { self.msg }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garbage() {
        let tests = vec![
            "<>",
            "<random characters>",
            "<<<<>",
            "<{!>}>",
            "<!!>",
            "<!!!>>",
            "<{o\"i!a,<{i<a>",
        ];

        for t in tests {
            let tok = t.parse::<Token>();
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), Token::Garbage(Garbage{ data: String::from(t) }));
        }
    }

    #[test]
    fn test_groups() {
        let tests = vec![
            ("{}", 1),
            ("{{{}}}", 3),
            ("{{},{}}", 3),
            ("{{{},{},{{}}}}", 6),
            ("{<{},{},{{}}>}", 1),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<a>},{<a>},{<a>},{<a>}}", 5),
            ("{{<!>},{<!>},{<!>},{<a>}}", 2),
        ];
    }

    #[test]
    fn test_score() {
        let tests = vec![
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];

        for t in tests {
            let tok: Token = t.0.parse().unwrap();
            assert_eq!(tok.score(), t.1);
        }
    }
}

const PUZZLE: &'static str = "Stream Processing";
const INPUT: &'static str = r#"
{{{{{{{<>},<<!>},<,>},{{<oou<o{!>,<i!!!!!>!!ouu}i>},<a!!!>!i!!!>!!<!!,!oa''!!!>!{>}},{{{{<a!!!>!!!>!!"!!o">}},{{{{{<!!i!!!>},<!!!>!>},<>}}},{}},<!>},<iu<!>!!o!!!!ee!!!!io!!!>!>!>},<'>},{{{{<!>,<ei'{!!}!>},<!><!!!>>},{<"a!>!>,'>,{{{<!>},<!>,<!e>}}}}},<!>},<!!!><!!i!!e"!>},<!>},<,!>,<!!!>!>a,>},{{{<>}},{{<!,u<!}!>},<!>},<i!>,<e>}}}}},{{<!>!o{a"i!u{}!>,<{!!!>,<!!!>!a>,{}},{{},<!>,<!>},<!!a,e{a!!!>e>},{<!eo,!!aa>}}}},{{{{{}}},{}},{{{{},<e!!!>>},{}},{{{<<!!ii!!!>i'!"a!!!>!>>}},{{{},<}{e}<!>},<>},{<!!o<!!!u!!"},!!,{!>},<!>!!!!!!!>!!!>},<>}},{<"e!>},<,o!!"!>e!>}"!>,<}a>}},{{<!!a'!!!!uo""!!'{!!!!'>}}},{{<,!!{ai{,ou!>},<a!>,<!!>,<>},{{{<'o!!!><!!!>u,!!!>!>},<!!!!!>,<>}},{{{{<>}},{{{<!!"aei!!!!!>"!!o"!!!!!>aioo,>},<!>!!!>,<!<,!!!>{i!!!>},<!<!!!!!!u'{!!,e"e>},{<{!>},<}!>,<!!!!o!!!>!!i}!!{!!!>>}}},{{{{<u!}<,o{!!!!,<{!!'>},<a>},{<<!>},<e!>,<'}u,!e}!>},<!!!>">}}},{{{<>}},{{}}}}}}},{{{<ae{,}<>},{{<!>},<}!eu}"},!a,a>}},{{{{<e!!ue"'<i!>,<!!ai!>},<"">},{}}},<,{>}},{{{}},{<!>!>},<!!!{!"'!uu!!!>},<{>}},{{},{{<<o!>,,i!!!!u!!'!'{"ei{">},{}}},{{{<!>,<}<!{,!>},<i!>,!!e}">}},{{<a!u!!!>!eu"!{!ua{>}}}}},{{{{},{}}},{{<!!!>!<au!!!!,!i"!!!>},<o}i,!!'{<'!>,<<>},{{{{},{<aiu,!>,<!!>}},{{{<!e"!a"i>},{{<!!!>u,a{!!!>,<eu}!!<}!>>}}}}}},{{<}!>u!>},<eooo!!a!!'!!!o{u!>},<>}}},{},{{{}}}},{{{{},{{{<{"o!!o!><'!!!>>},{{<!!e!!,}ia!>},<'!!,'<e!>},<u!>!>,<!!!>!>,<!!!>>},{<!u!!,u<!>},<!!!>u!!!>!>,<u>}}}},{{{{<u!!},,u{'!!!!!!,!!'>},{<<',!oi!>},<!>,<!oi!>!>},<e>}},{{}}}}}},{{{},{{{{}},{{},{<!>},<!!!>!!!>"<i!!>}}},{{{<!!!>!>},<u!,i!!!!!>!>},<aa!!{!!!ie"!>!">,<!!!>!!ee>},{{<!><!!!><{!!!>'e!!<!>,<!>,<'!{>}}},{{<}u,!,!!!>!>},<!!!>!"i{!!!>,>}},{{<"{!!!>ioua!!!>u!ouu!>a!!u>}}},{{}},{{{{},{<e>},{{<!>!!!!!!!>,<"!!!>ea<aa!>},<'!!!>ue!'>}}},{{{{<!>,<ei!!!>},<o!!ei!"!!<!>},<u}>}},{{{{{},{<i'!>},<!>,!>},<"oa<>}}},{{<au"}!>,<!e"ieau>}}},{<",o<!>!!!>e,!>,<uo!>},<eae!>},<",!!!i>,{<e!!<!!<uiu>}},{<!,<}ae}!>,<ue}i}!!!>{>}},{<!!!}'!,o}!'iu!>},<!!!>!'o>}},{},{{{<'>},{<>}}}},{{{{{},<>},{<{!oo"!>},<<"e!>,<!>"{o!}!!!>},<a!>>}},{<!!!>,<,"e!!i{ii!!!!a}u>,<!!!eooio!!!>!>,<'o{!>!>,<!>,<,>},{{<'!!!>!i!!!>"!>},<ieo!!!!!>,<!>'!>,<u!>},<}>}}},{<!>},<"}!>},<!!o>,<uo!!{<'!{a!>,<>},{{{<!>},<'u!>},<>},<!!}e!>'a!!!>!o!!}">},{{<i'{{}ea{"e{!!!e!!!}!>>}}}},{{<<!!!!!e!!{"'!>},<!!!!'u!!a!"!>,<!>},<<oo!!ie!!!>>,<u!!!>!!a{>},{{{{<>}},{}},{<!!!!a!>,<"<<<!!i"!!'"}!!!au'>}},{{{},{}}}}},{{{<{}a!u!>},<!>},<ei!!!>{!!'>},<!!!!!>!!!!!>oo''<>},{{<!>},<!!!!!!,!>,<a!>},<'>},{{<u{!>!!!>!>},<a!,ae!!!!!>{>}}}},{},{{{{{},{<u!>,<!>oio!}!>,a!!'u!>},<'!!>}},{<u!>,<!>!!!>},<}!>!!!>!!!!<{!!io!>!!!>},<!!io>},{{<i{!>!!!>>}}},{<i>,<i!!ou'!>},<!!!}ea!>!!"'"{!!,>},{{<,"!!a!>},<!>,<!>},<!>},<!!!>>},<{!!ou'!!!,a>}},{{<!!o',e!!,a>},{}},{{{<!>},<!!!>o{u"!!!!!!!!!!>}},{<!!!!!>>}}}}},{{{<e!!},au!!ee!a!!{!!u>},<o!!!{!"!!"i>},{{},{<!>,!>,<>,{{<!",e<o'<iao,i!>,<!!!>>,{<i{!!!>!!!>!>!!!>!>},<!}a}!ioe>}},<}!!e>}}}}},{},{{},{{},{<uii",i!!!o'">}}},{{<}!>},<a!!,!>,<!!!>!!!>a!!"!!'>},{<{'!>},<oau!!!>i!>,<!!"<{!>'!>},<'!!>,<>},{<!a{o!>},<!{!a>,{<u,"!!!>,>}}}}},{{{{{{<""!!!>!>,<'!o!!!>>},{}}},{<'!"o!!!>}!>},<>,{<'>}},{{<,a!i"!}i!!!>"a"!!!!{u!!,i>},{<o!>},<!!!>},<{io!>e!>,<!!!>,<>,{{<o'!!ioa!!ia"""'>}}}}},{{<!!!>!<au!!,!!"e!!!>!a,eeia,>},{<>}}},{{{<>}},{{<!>},<}>},{<o'a>}},{{<<!<!>},<{!>!ia!>},<{o!>}!!!>},<>}}},{{{<!>'o"a}euu!!!>ua>},{{<"!>},<{!>a!>},<i}!>,<eo!>},<>},{<}!!e!a!!>}}},{{<!>},<!!!>,<!!!<i"!!<!>,<!>,<!>,<!!}!>,<<oo>,<!>,<"!!!>,<!>},<aiio!!,!>}!u!>},<u!>,<u!!"a{>}},{{<>,{<!oi>}},{{{<}i!'",!>!,!!e<!!!>!u>}},{}},{<ie!>>,{<!>,<"'!'e!!!>!>},<!>},<u"!>!!<{>,{<e'!>,<!>'!>},<'!!!!a{{!!!>!>e}a}!>},<!>,<>}}}}},{{{<!!!>!>,<!<''>},{<o!!!>!,!>},<!>u''<>}}}},{{{{{<!>e"a>}}},{<!>!!}!>,<'o!!!>,<!!!>}""o{u!!!>>}},{{{<!!au"!!}!!!!!>},<{!!e,o!>!ou>},{{{<!!>}},{}},{{<"'!'{>},<!>},<o>}},{{{<!>,<'a,!eo,!!>},<""!>,<!>},<!,'!!!><!i!>,<e{>},{{<!!a!>!!!>ii!>},<!>,<u,!!i'!!i'!!<!!{!!!ea}!!!!!!>,{}}},{{},{{<!>,<!>},<!}<,{ua,<!>},<e!!e<>},{<!!!>uo"!!!>,}e!>u>}}}},{{{{<}!!}!>}'!!<{!a!>,<a!{!,>}}}},{{{<{"<!ua{!>},<"!ii"'}!!!>!!}<!>,<<>,<,!!!!!!!!!>"{!!"eu,!!!>,<o"a!!i!>,<e>},{}},{},{{},{{{<u!>u<!!<!>},<!>,<!!!>}">},<""e}<,!>!o!!!>}!!o>},{<{>}}},{{{{{<!a}uuee>}},{{{<,o!>},<u!!!>!>,<!}!!!>a}ee>},{{<{,!!,!>},<!!!>a>},<!><'<i!>!!u<a!>{i>}},<!!'oe!>},<<u!!!!>}},{{<aa!>},<!o!>},<"{o<>},{<!>},<!!!u"!>,<>}},{<}oio>,{{<!<{}<"e!!!>'!>},<o!"i,!i!!!!u>},{<!>,a!>!>},<!!!>,<}>}}}},{{<{,ei!e{'{!{">,{<i!!!!!>,o!",a}!>,<!!ia!!>}},{{<",'!>{!"iu!!!>!>},<!>},<!!!>>,{<"a!!!>!>!>},<}{!,!!!!ou}{>}},<!!ie<!!!!uuu!u,!>u}!!!>>}},{{<"!>,<!>,<!o{!>},<!!!>a!!a,!>,<u""a!><">,{<!!o!!<'!!!>!>,<!!!>!>},<!!ae!>},<!!!>},<!!!><'>}}}}}}}},{{{{{<!}!o{!e!!<<o!!o!!,!>},<e>},<i!>},<"o!>!>},<'!!!>,<a!>},<e>},{{{{<au,'!!!>!>,<!!a>},{<"!}!i!!!>},<o!>},<o}'!!!>!>},<{>}}}}},{{{<i!>"!!ua!>},<"!!!>!!!>{'ie{'}>},<!!!>,!!!!!>},<u!>!!u"{'o>},{{<u!!!!!>!>},<!!a!a!!{{'<!!!!!>,<!e>,{<!!!!!>o'ui!!!><{>}},{{},{<!>,<<u!>{o!!}"!>},<{}e">,{}}}}},{{{<ao'"!!!!!>!>},<!!!'"u>},{{{{<!!!>,<>}}},{}}},{{<}!>,<'!>,<!!iu!u!!!>,<!}ae}!i!!!,!>!>a>},{<!!!>!>,<"<e}!>,<<!!i>},{<!!!>!>,<!io>,<{!!"eo>}},{{},{{<!!,!!!>o!io!>},<!!!>'<!!!'!!i>},<'i,!>iea!!!!!>"!!!'>}},{{{{<e!>,<}!!"!>,<<a!e'}<>}},{<!>},<,'!>,<u>}},{{<o!!!>,}!!'!>},<!>},<{>},{<aa'u!!}!>>}},{{{{<!>'!!"i!!!>!"o!!!>!!!>ii'>}}},{<u!!!>},<a!>,<u},!>!!!!!>!>,<a<!{!>,<{<u>}}}},{{{<!i!!!>a}">}},{{<u{!>a!>!>,<{!!!!i!!!>,<!!!>,{>},{<u,e!!,<i!!!au!>},<'}}!>,<!!}o>}}}},{{{{{{},{<!>},<'{!!>}},{<a!a!!!>!!{!>o!!e!!"'!!!>},<"o!!<>}},{{<!!}au"!>>,<!eo!>,!><!!!!'i<,!!!>,<}!!<{!io}>},<'"!>,!!!>!>},<e,eei!!}!!<!>,<o!!!>>},{{<e<aoaua,!!!>aa!!!>eau,'>,<i!ua"u}>}}}},{},{},{{{<o'!'o!e!!!!'ea}>,{}},{{<,{<e!!!!}}<!>},<>},{}},{{{}},<!!!>!>!!"'!!u!a!!,a>}},{{},{<!!!>oo"!!!>,{>,<!>'<!>>},{<o,!!!!!'!!!>!!!>!!'{''!>{i{!>},<>}},{{{<!>,<"!!'i"u<!{o<i{i!}!>i>},{}}},{{{{<'<{<iaa!>},<!>},<{a!!!>>}},<o!!!>!!{!!o>},{{<!!!>},<i!>!!!!e!>}!!!>!!!!!>'">},<>},{{<'{!!}a">}}}}},{{{{{<<!>,<"!>},<u!>},<!!!>,<!i'{>}},{<,!!!>},<au!>},<!!ie,!!}!>,<>,<,!>},<!!!>,<a>},{}},{{<i"u,!{ae{,!<a'!>},<!>,<>},{{{{{<o"!>ua,!}'<<u!a!>,<u}!>>},<i!>},<i!!!>o"!>},<">},{{{},{}}}},{{<!!o!,,!a}>},{{<>},<!!"!>},<!{"}!>},<!!>}}},{},{{{{<!!a}!!e!<uu!}!!eu!!{ai{!>!'>},{<'!!!>!!!>},<!!!!!>"ioa!!!>}!!!>'>}},{}}}}}},{{<<!>,<!!!>!o'e{!!""!e',a!>!!!>},<>},{{},{<!i!!ui<""}!!!{oi,!>,<<{o!!u>}}},{{{}},{{<{>},<!!{!!!>,<!u!!,!>},<i!!ui!>o!>a,>},{<!>o!>e'!!!>},<!!<>,{{}}}}}},{{},{{{<o!!!><,!!"}!>},<!!!><u!>,<>},{{}},{<!}{!!!'a>,{<!!!!!!au,!>},<i!>,<!>,<<aaa!e>,<}!!!!!>e"{!!<,'!>!>!!!>"!ee{!>},<>}}},{{{<a!>,<!o},ai!>},<>}}},{{{{<!>,<e<!>},<,e!!!>}'!!'uei>},{<"{i>}},<!,!{!>},<"!>,<u!!!<!!!a!!!>eu!<{a>}}},{{{{<{"!oi>,{}},{{<}!>},<"au>,{<!!}{io<!a!!!>,<<ai!!{!eo>}},{}}},{{{<!!!>i!!!!}'e!!!>"}<!u!}a}>,{<<!'"ea!!!>!!!>>,<>}},{<!>},<au!!!>},<e!!o!!a,!>!o!>,<>,<!>,<}!>},<!>!!!>!!!>!>},<>}},{}},{{{{{},{}}},{{<!>',ua'!!a{{>},{<!,!!!>{>}},{{<}!>aai!!'"}!<!!!>!!<!!}uiue>},{{{<!,!>},<{!!e!>},<!e{!!}!!u<a"!>,<u"i!>,<>},<>}}}},{{{<o!>},<!!!>e!!{'"oi!>>}}},{{{<!!},!!!a!!!>!!i,o!>},<!!!>>},{{},{<!!ii,!!!>,<"!!!>}!>},<!!u>}}}}},{}},{{}}},{{{<!>o!e,!!>,{{},{}}},{{<!<>},<}!>},<u'u<ea!i'<o<!>oeee>}},{{{<i,{a<'!>i"e{!>},<o>}},{}},{{{},{<!>u'!!!o}a!>,<>}},{{{{<,oo!!!>o<"ao!!!>,<'!}!u!!,,{>}},{{<u!!u!!!><!!!>},<},>},<i"!!'e!>},<u!>},<!!!!!>},,!>{>},{{{{<}"""e!>ei"!"}!!e'!!>}},{<!!!>},<{<}}ieoai!",!>,<u>},{{{<!!,{!}!i!>},<'}!}u{,!}{e!!>,{<,}}!>!>!,i!>},<!{"o}u!!"i!!a}>}},{<a!>o,ua!!!>,<u!>},<"!!!!a!}a!!!>},<{!!!>!>,<>}}}}}}},{<!!!>!>},<i<!!!!!>},<e!>o}!!!>,<{!!<}u>,<!!!!!{!!o!ooe!!!!!>!!}}!>!>},<a!},!!!}'!!,i>}},{{<aa!!!>!>,<a'!!!><!>},<!!!>},<{!!!!!>'o!>,<>},{{{<,<!!!{<eaa!>'>},{}},<a{!>!!},!!!>'>}}}},{{{{{{<!{e!>>,{<!!!!!>!<!>},<!!!>,<!!!>,<>}},{{<!>},<!!!!!>a,!!}!"i{!!!>,<>}},{{<o!>,<!>}!!!!>}}},{{{{{},{<!>,<e!!!!!>}!!!>,<!!!>,<!!iei>}},{{<!!!>e!>!>i!!}!>,<>},{<i!!!!!>,<,!!ou!!<!!<e"{e<>}},{{}}},{{<!u<!!!><}u'!!!>},<,!!!e>}},{}},{{<!>,<a!!i>},<}u""!!!>io!!!>},<!!e!,!!>},{<!!!>{>,{}}},{{}},{{{<!eu!!!>o,!!e''i<!!!'!>},<!!!>>}},{<!!!>a!>,<ao!!!>,<>,<,!,}>},{<!!!>o'!>e>,{}}}},{{{{}}},{{{<u{,'">,{}}},{{{<o,!!!>!>>}},{<'o}'!>,<!i>,<!!a!>,<!!!!'{!!!>{a!!i'>}},{{},{{{<,!>},<!!!!!!,!>},<!>,<!>,<i>}},{{<!!!>u!!!>o!>ei,!}!!!!!!!>!!!>!>,<>},<'o!}'>}},{{{<i!>,<<>}},{{<!!{!>},<!>},<!!!>},<!!!>,<u!>},<!>!!!>ae!>,<>},<!!!>u!>,<<'<!"}!!!!!>{!>!!"}!>,<<>}},{{{<!',<!!ua!"'!!a!>},<,!o!!u!!!}'i!!>,<o!!o>},{{{},{<!!i!u">}},{}},{}},{{{}},{{<"u!e!!,!,!>},<i!>},<i'{!!!>,a!>},<"!>,<!>>},<!>u!>},<!!{!>!>,<!><}!!i!>,<i'a'!!,e>}},{{{{{{{},<}}},!,<e}!!!!}!i!>>}}}},{{{<a}e!>},<<!!!>>,{{<!!!!<,{e!>!!!>!!a",!>},<u{>}}},{<!!!>"<e<!!oo!!!>!>a!!,!>},<>}},{<"!!!>},<,!u!!!{!>},<,<,o'!>,<"!"{!>,<>,<!>!e"!!!!!>!!!!}"<<!!!!!>'o!>},<"!>},<}u>},{{{{<,!>,<a,e!!!>!<!!,>}}},{{<!io{>}}}},{{{}},{<!>,<!!!<!!eu'!!!>}!>},<,i{!>},<!!!>>,{{},{<!!}'!!!>i!>,<i!!!}!>!>!!i!{!>,<!!!!o!!!>},<!!!>>}}}}}}}}},{{{<>}},{{<>},{<!>,<oa'o!!e<,!>,<a!>,<>}}},{{<!!e!,i!<"!<<!>},<}!!!>!>>,<!!!}e!!!>,<o!!!>!!>},{<o'o!>!>},<e{!>!>!!!>},<!!!>!e!!!>!>!>o!!"!a>,{<!>,<!!!!!>>,{<'>}}}}},{{{<u!>!!,e!!>}}},{{},{{<!>},<{ia!!}e"<!>!>},<!!i"{!>,<!!e!">}},{<!"}'!>,!>},<!<!!'!>u!'!!i!!!"!<>,<!>!!ua}<!!<u"!!!!!!!>{}>}}},{{{{{{{},<>},{<!>!!!>!!a!>,<!!!><>},{{{<,,ua!>,<<"!i!{"!>},<oi>,{<>}},{<}}!!!>!>o!!!>!!a!!{i<ua,{a{>}},<<e!>,<!>},<i!!!>!!!>}!!'!!!!!>,<u<!!<!!!!}!!u>}}},{},{{{<!!}e!>,<>}}}},{{<!!!>!!!>,<ii!>,<!!!aaaa'!>},<!!,!<!>,<!>!>}>,<!!u!ui!<!!!>},<<<!>,<ia!>e!!u!!{<>},{{<uuia"}u!>,<o!>,<{!>},<!!!>!'!>},<'{,">,{<!>},<<,ue!!<!e!>,<!>},<!!!a>}}},{{{<!!!>"!!e!>,<!>,<{!>,<>},<!>},<'a"!!!,!>},<!!'>},<'"uo{e!!{!!!>i"!>!{!>},<,i{>}},{{{}},{{{<!!!>!>,<>,<iou!!!>!!!>!>,<!,u!!!>!>},<>}},{}},{{<!>},<!>},<!>i<!!o{u!!!>,<a!i!!!><>}}}},{{<,oa!>o"!>},<a!!a}>},{<'!>},<ii">,{<,u>}},{{<a!!!>,<o}!!!>,!<'>},{{<!!e!!,!ui!o,>}}}},{{<>}},{{{},{{<!>},<<!,!!!>!!!!!!!>},<e!<!>},<!>!!'>},<o">},{}},{{<"!i!!ii!!!>,<,!!!>'{,'u<>}},{{<<!{'"!>},<>},{<,!e!>!!!>i!>,<!>,<!!"!>},<!>!}!>!>},<oa{">}}}},{{{{<<>}},{{<!>!!u!!!!},!>},<!>'<!>,<!!{"i"'!!!>!>},<o>},<!>,<!o!!!>>}},{{{{<}{a!>},<uu}<!>},<!>},<,"ia!'!!,e'i>}},{{<!!>},{<!>},<{!>,<ii,e'a!>},<!!"!>!>,<">}}},{<!>,<{"!>!>,<!i!>>}},{{{{<ea!!}i'>}},{},{{{{<!!!>!!"a!>,<>,{}}},{<!>},<e'>}}}},{{<!!"'{{!!o,'u!,>,{<a!!{!>>}},{{{{{<}"!!!>!>u!!!>}}i!>,<}'a!{{>},<,!>!>!}<i!>,<!!!>},<a,>},{{{{<>}}},{<!>,<!!eao,!!!!'>,{<i!!'",!",a!au>}},{{<!!',!!oa!!!>a!,ei!!!>!>"!>>}}}},{<!!,!!!>!>},<!>!>}}""<!!,eo!>},<>,<a!!!>{!>},<!"<!'}>},{<o>}},{{<e}!>},<u!>!>},<!!"!!<'!!i!>},<!>,<!}!>{,!>},<>}}},{{},<!>!!!>'}eu!>,<!a!!!>!!!>!!!>!!!>"}!e>}},{{{{<!,!}!!!e<!!!>!>,<i!i>},{{<!!!>,<{!{i!>}!}a",oo!>,<!!!>,<>,{<,!e'>}},<!>},<!i!>},<!<<!!!>a!!!!'<'!!!>,<>}},{{<e!>},<!!!!}>}},{{<o!,{o!!!!,!e<i!>,<u'<i!!!!e>},<!u!>,<!>},<!!!>},<"i!!e!>},<iu}!>},<i>}}},{{{<!>{}"!!!>},<!!u<!>,<"!<a,{!!!>>},{{<!>},<>,{<a"'!>},<{"!}!!oo!!!>'!>,<u'!>,<'>}}}},{{{<!!!!u"ioa,!>},<!!!>!>},<!!!>},<!'!i!!u>},<{!>,<u!!<}!!!{auau,}i>},<a<!!ou!>,<!!!!i{!>,<<>},{{}}}}}},{{{{<!!u,'<!!!>},<<u!!!'!',!>,<"!>,<!!!!!>}>},<!>,<!!!!<,!!!!!!}!>{'!>},<ue!>,<!ao!!!!!>>},{{<!!}uo!>,<}i!{>}}},{{{{{},{}}},{{<a!>{>,<!>,!!!u!>>}}},{}},{{},{},{}}},{{{{{<a!>,<e,!!"!!!>},<!!{}}!!,io{!ioou>}},{{}},{}},{{},{{},{{{}},{{<<!!!>},<,'!!a!>},<!!!>'!!!>>,<!>',!>,<e,<,<uiaaa!>!!'!>},<>},{{{<!!!>'!!!>!>},<e!>ue!e"!!!>},<<uu!!!>"o>},{<!!!>oe}{'!!<>}},{<>}},{{},<'!>},<!!!>u>}},{<>}},{{{{},{{{<,a!>e!!!>!!!>}o!>},<e!,!!e}"!!!>,<>}},{{<}!!!!!!!>o<>}}}}}},{{{},{<{!!!>'!!u!!!>'>,{{<!o!>'!!!!e!"i>}}}},{{{<u>,<!!,o!!e!!o{o!>,<>}}},{{{{{{<!!!>}!>},<!!{!!!,>},<!!!!u!>o''"u>},{{{<!!!>!!a!>,<'{!>,<!>,<>}},<<!!""!!o!!iauu!>},<i!!!>ii!,!!e>}},{}},{{<!!!!!"!>},<{i}!>},<!!e{ea!!!>!"<e!!u>}},{{{{}},{<,!>i!!i,a!>},<"!>u!aoe>,{<,<!!>}}},<i!>!>,<'!ua!o!>,<!u!!>}},{{{<ea"!!!>!>},<!,,!!!>!!oie"!i>,<!!!>{!!!>'{{>}},{{{{<!>!!o!>i!!!>},<<e!!'!>,<e>}}},{},{{<!>},<!!}>},<uo!>>}},{{{<'!!!!!>o!!!!!>!>,<!>},<!!!>>,{<}!!!!!!u!!!>},<!!!>,<!{!!"!!!>>}},{<>,{<eii{!!'eua"i!!!>'>}}},{{{<e!>ee}i!a!!!>!!!>},<}<!!!>!!,!!!>!>!!i!>,<i>}},{{{{},{}}},{{{{<!!!>e{uo{u>,<!>!!!>,<{!>}!u>},<!!!>'>},<,!!u!>,<!!}aiua!!!>,<!!!>>}}},{{}}},{}}},{{<,{{>},{{<,!>},<i!!!>iu{}!>,<!!!>!!!>!!!>{!{a,>,{{<!!!>!aa>},<au"ua'e!!!><!>},<!u!!!>o!!'!,>}},{{},<"'ai!!!>'ia>}},{{{},{<u!>},<"!!!'!>!!!!}!>},<!!!>!!!>,<uo!>,<>}},<!>},<}'!><o{!>,<i{!>!!,>}},{{{<!!!>"}>},{<ie!>,<}}ieo!!o!>,<!!!>!!!>>}},{<"!>!e!!!>!>},<!!{,!>,<!!!!!>,!<>,{<>}},{<!>},<!!a}!!{"!>,<!>!<}<o!!!>{'>,<!'"o>}}},{{{{<!>,<!!!}!!}!>,<eo<}>},{{<e!!!>,<!e}!e{!!!>},<!>o!!<!!'!!!{{<!><>,{}}}}},{{<}'!!!!'!<o!>>}},{<,u!!{!!!!!>"!!e>,<!'<!"!>},<}"}<!!!>!!{o!!!>o!ie!!!>>}}}},{{<!!!>},<!!!>i!>e<!>!<!>,<!!!!{"uao"i!>,<!>},<,>,<u!!{{!!'o!>},<!!,}>},{{<}ei!>,<o{!!!>!}u!>},<!>},<!!o{!>,<iui!>},<>},{<!,oo''}!!!>},<!!!>i!>},<a!!!>u}u>}},{}},{{{{{{<"io!>},<!>,<"}!"!!!>}}>},{<>}},{}},{{{{<!>!>},<!>,<!!uo!!o!>,<!!eu!>o!{>},{<a<iu,!!aa!ea!!!!!!!>!u<i<,!!"!!!o!e}>}},{{<u!>,<e<>}}},{<a!>},<!>e'<!!!!!>!!!!'!a!>},<}i>,{{},{<<a!}!o"!>,<<u!>},<!>},<!!!>!!a"e>}}},{{<!a",i<>},{<!io!>,<>,<'ai!">}}}},{{<e!>a}}!"!!e!!o,e!>,<i{!!!>,<{<ua>},{<!<i"!!!>!!>}}},{<!!!!!!u>,<e!!!>,<<eu>},{{{<!!}{!>},<''!!!>!!{!!!><!!i!>},<a,!!!>},<'!!!!!>},<>},{}},{{<!!!><!!ou,!>uea>},{}}}}},{{}},{{{{{{<u>}},{{},{<}!!!>,<u>}},{{{<!>u!{io!>,<!!a!!!>{{>},{{<}!ea!>>}}},{<!!!!!>,}!!!!!>!!!},!>,<,!!{'!!!o''<>}}}},{{{{{},{{}}},{}}}},{{{}},{},{<'u!!!>!!!!!>},e}{io>,{}}},{{{{}},{<!!"}>}},{{},{{{{},{{}}}}},{{{},{{}},{<{!!!"!!!>!ea!!!<>,{}}}}}}}}},{{{{{<!>,<a!>!>!"}!!>}},<!!!>!!!!!!!"!"i,!>,<>},{<!!!!!!<!!!>i!!!>{e!>a'i{!!!><i>},{}},{}}},{{},{{{<!uo{{o!!!e!>},<!"eo>,{<!!>}},{<u,>,<,'aa{!!!>},<!!!!!!a!!'',i}'!>!>},<!!}ei>},{{<!>},<!>e>}}},{<{!!!>!>!!>}},{{<!!!>,<"{oo{!!!>,!>},<i!>>,{<i<"iu!>a'!!!!}!>,<o!!!>ii!!"!>>}},{{{<'oo"}!!!>},<ee'!!!>{e!!"!>,<o>},<iui'!!!>},<oa>},{<o<u!!o<!!!>!!}uu!>},<i>}},{{{}},<{<!>},<i!!!>u<u!>},<!>!>,<,{<,{>}},{{{{{}},<}o!{<}eoe!>'{a!>,<!>},<>},{{{<{!>,<!>!}!!!!}>,{<ue{!>,<!>},<!>},<!!!>o!!'!>,<!}!,>}},<"!>!>e!>},<"">},{<i}!!u!!!!!>,<!!!>!!a'<!>},<!>},<{"!!!>!!>,<!!!>!!{>},{{{}},<oeae!>,<!"!!!>'!>,!!!><!}!>},<!>,<u>}},{}},{{{{<"'"},'}!>},<"!,!!!>!>,<!!!!!>!>,<!!a>}},{}},{{<!"!!,!>},<!!{'<u!!!>{!><!>},<!>ie!>,<!!!>},<>}}}}}},{{{{{<!!!>au>,{<!!!>!>{"!>,<!>},<{{a'a"{,o!!!>!!<>}},{},{}},{{<>}}}},{{{<!>!!!>!!!>>,<!ie}!!!>>},{{{<"ei!!!>,u>},{<<<!!!>,<!!!>ia!>},<!!!!!>"">}}},{{{{<a!>,<u{'!i!>},<!o>},{{{<eu!!!>},<>},<!!!!"!!!>!!!>i,!!!>o"ao!>,<e'!!i>}}},{<'!!o">}},{{{<>},<!>a>},{{<!!"!>,<u!>,<!>,<}>,<!>,<ou{!'{!,}!>,<>}}},{<'<e<{''i!>},<!!!>,<u!>,<!>,<<<"!!<>,{<!!aoa!!!>!!'!>{!>},<'i!!!>>}}}},{{{{<'e}u"}a!!!>!!!>,<!!!>},<}!!!>>},{<}e!!!!}a{i>}},<i<a">},{{<<!>,!>!i!!!>},<!!eu>,{{<!!!>{}!!"!>},<!>!!!>!!!>"!!!!>}}}},{{<}!!>},{<i,!>},<!u,!!u>}}},{{{<!>,<!!!>!!!>!!!>,<}!!>},<!>},<uo<!>},<!>},<i!u!!!>!}"<!!!!!>o,>}}},{},{{{{},<!>,<,eeu!>,<,"u'"!>},<">},{{{<!!!!a!a!>,<!>,<!>},<!>,<!!e!>,<>}},{}},{{{{{}},{{<ea!!e!>oo!>o!!!!!!!>{!!u"'">}}},{<o,}!!e!>},<a!!!!u!!'{io>}}}},{{{<>,<'a,}!!!!!<'!!!>},<e!!{!>,{!!"!>!>},<!!!}>},{{<a>},{<!!}"!!!>},<!>},<o<o!>},<i!>},<!!!>,<!!!>"!!<{!!o>}}},{{<!>e}{!{!>,<!>,<{<!!!}!!!>u}e!>e!ua>},{{<"}'!>,<u!!!>{,!>}}{!!!!!>,<!!{}>}}}},{{{},{{},{<'!!!>}!>},<oa!!o"!>},<!>,<!'!!!>,<u!>,<o!>!!<!>>}}},{{{},<!>!>,<,<!!o}{>}},{{{<>}},{<!"ou!,,i!>,<a'!>"'!><{>},{<,!!,!!!!!>"!!!>!>},<"!!!>!>>,{<!>,<!>},<,!>},<!!!>}!!"!>,<!>},<<<!>u'<!!!>>}}},{{{<}{!>,<<"!!!>!!{!>,<>},{{},{<"!>},<{e}>}},{<"a!>},<e!>},<'{!!!!!>i!>,<!>>,{<!!!>,{,!!!>!!a!>},<!>},<>}}}}}}}}
"#;
