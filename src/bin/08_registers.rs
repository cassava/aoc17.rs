/*!
Day 8: I Heard You Like Registers

You receive a signal directly from the CPU. Because of your recent assistance with jump
instructions, it would like you to compute the result of a series of unusual register instructions.

Each instruction consists of several parts: the register to modify, whether to increase or decrease
that register's value, the amount by which to increase or decrease it, and a condition. If the
condition fails, skip the instruction without modifying the register. The registers all start at 0.
The instructions look like this:

```text
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
```

These instructions would be processed as follows:

1. Because `a` starts at 0, it is not greater than 1, and so `b` is not modified.
2. `a` is increased by 1 (to 1) because `b` is less than 5 (it is 0).
3. `c` is decreased by -10 (to 10) because `a` is now greater than or equal to 1 (it is 1).
4. `c` is increased by -20 (to -10) because `c` is equal to 10.

After this process, the largest value in any register is 1.

You might also encounter `<=` (less than or equal to) or `!=` (not equal to). However, the CPU
doesn't have the bandwidth to tell you what all the registers are named, and leaves that to you to
determine.

What is the largest value in any register after completing the instructions in your puzzle input?
*/

extern crate aoc;

use std::fmt;
use std::error;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 8: {}", PUZZLE);

    let prog: Program = input.to_str().lines().map(|s| s.parse().unwrap()).collect();
    let mut reg = Register::new();
    prog.iter().for_each(|x| { x.apply(&mut reg); });
    let max = reg.iter().max_by(|x,y| x.1.cmp(y.1));
    match max {
        Some(max) => println!(":: Answer 1 is {}={}", max.0, max.1),
        None => println!(":: No elements found"),
    }
}

type Program = Vec<Statement>;

type Register = HashMap<String, isize>;

#[derive(Debug)]
pub struct Statement {
    condition: BinaryOperation,
    operation: BinaryOperation,
}

impl Statement {
    pub fn apply(&self, reg: &mut Register) -> Option<isize> {
        if self.condition.apply(reg).as_bool() {
            Some(self.operation.apply(reg).as_num())
        } else {
            None
        }
    }
}

impl FromStr for Statement {
    type Err = ParseError;

    /// # Safety
    ///
    /// This function assumes that the string comes in this format:
    /// ```text
    /// c dec -10 if a >= 1
    /// ```
    /// That is,
    /// ```
    /// {ident} {op} {operand} if {operand} {op} {operand}
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<String> = s.split_whitespace().map(|s| String::from(s)).collect();
        if tokens.len() != 7 || tokens[3] != "if" {
            return Err(ParseError::with_msg(s, "length is not 7 or token 3 is not if"));
        }

        // TODO: Put this all in one statement?
        let target_op = BinaryOperation{
            op: tokens[1].parse()?,
            left: {
                let ident: Operand = tokens[0].parse()?;
                if !ident.is_ident() {
                    return Err(ParseError::with_msg(s, "left operand is not an identifier"));
                }
                ident
            },
            right: tokens[2].parse()?,
        };
        let condition = BinaryOperation{
            op: tokens[5].parse()?,
            left: tokens[4].parse()?,
            right: tokens[6].parse()?,
        };
        Ok(Statement{
            operation: target_op,
            condition: condition,
        })
    }
}

#[derive(Debug)]
pub struct BinaryOperation {
    op: Operator,
    left: Operand,
    right: Operand,
}

impl BinaryOperation {
    pub fn apply(&self, reg: &mut Register) -> Constant {
        match self.op {
            Operator::Eq => Constant::Bool(self.eq(reg)),
            Operator::Ne => Constant::Bool(self.ne(reg)),
            Operator::Le => Constant::Bool(self.le(reg)),
            Operator::Ge => Constant::Bool(self.ge(reg)),
            Operator::Lt => Constant::Bool(self.lt(reg)),
            Operator::Gt => Constant::Bool(self.gt(reg)),
            Operator::Inc => Constant::Num(self.inc(reg)),
            Operator::Dec => Constant::Num(self.dec(reg)),
        }
    }

    fn eq(&self, reg: &mut Register) -> bool {
        self.left.resolve(reg) == self.right.resolve(reg)
    }
    fn ne(&self, reg: &mut Register) -> bool {
        !self.eq(reg)
    }
    fn lt(&self, reg: &mut Register) -> bool {
        self.left.resolve(reg) < self.right.resolve(reg)
    }
    fn le(&self, reg: &mut Register) -> bool {
        self.left.resolve(reg) <= self.right.resolve(reg)
    }
    fn ge(&self, reg: &mut Register) -> bool {
        !self.lt(reg)
    }
    fn gt(&self, reg: &mut Register) -> bool {
        !self.le(reg)
    }
    fn inc(&self, reg: &mut Register) -> isize {
        let id = String::from(self.left.ident());
        let n = self.right.resolve(reg);
        let r = reg.entry(id).or_insert(0);
        *r += n;
        *r
    }
    fn dec(&self, reg: &mut Register) -> isize {
        let id = String::from(self.left.ident());
        let n = self.right.resolve(reg);
        let r = reg.entry(id).or_insert(0);
        *r -= n;
        *r
    }
}

#[derive(Debug)]
pub enum Operator {
    Inc,
    Dec,
    Eq,
    Ne,
    Lt,
    Gt,
    Ge,
    Le,
}

impl FromStr for Operator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operator::Inc),
            "dec" => Ok(Operator::Dec),
            "==" => Ok(Operator::Eq),
            "!=" => Ok(Operator::Ne),
            "<" => Ok(Operator::Lt),
            ">" => Ok(Operator::Gt),
            "<=" => Ok(Operator::Le),
            ">=" => Ok(Operator::Ge),
            _ => Err(ParseError::with_msg(s, "unknown operator")),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Ident(String),
    Const(Constant),
}

impl Operand {
    pub fn is_ident(&self) -> bool {
        match *self {
            Operand::Ident(..) => true,
            _ => false,
        }
    }

    pub fn ident(&self) -> &str {
        match *self {
            Operand::Ident(ref s) => s.as_str(),
            _ => panic!("cannot unwrap Operand"),
        }
    }

    pub fn resolve(&self, reg: &mut Register) -> isize {
        match *self {
            Operand::Ident(ref s) => *reg.entry(s.clone()).or_insert(0),
            Operand::Const(ref c) => c.as_num(),
        }
    }
}

impl FromStr for Operand {
    type Err = ParseError;

    /// # Safety
    ///
    /// This function assumes that `s` does not have any spaces.
    /// If this precondition is not fulfilled, the result will
    /// be an `Ident` containing strings.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<Constant>() {
            Ok(Operand::Const(c))
        } else {
            Ok(Operand::Ident(String::from(s)))
        }
    }
}

#[derive(Debug)]
pub enum Constant {
    Num(isize),
    Bool(bool),
}

impl Constant {
    pub fn as_num(&self) -> isize {
        match *self {
            Constant::Num(n) => n,
            Constant::Bool(b) => if b { 1 } else { 0 },
        }
    }

    pub fn as_bool(&self) -> bool {
        match *self {
            Constant::Num(..) => panic!("cannot convert number to bool"),
            Constant::Bool(b) => b,
        }
    }
}

impl FromStr for Constant {
    type Err = ParseError;

    /// # Safety
    ///
    /// This function assumes that `s` does not have any spaces.
    /// If this precondition is not fulfilled, the result will
    /// be an `Ident` containing strings.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<isize>() {
            Ok(Constant::Num(n))
        } else {
            match s {
                "true" => Ok(Constant::Bool(true)),
                "false" => Ok(Constant::Bool(false)),
                _ => Err(ParseError::with_msg(s, "not a constant")),
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    msg: &'static str,
    data: String,
    column: usize,
}

impl ParseError {
    pub fn new(data: &str) -> Self {
        ParseError {
            msg: "malformed input",
            data: String::from(data),
            column: 0,
        }
    }

    pub fn with_msg(data: &str, msg: &'static str) -> Self {
        ParseError {
            msg: msg,
            data: String::from(data),
            column: 0,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: at column {} in: {}", self.msg, self.column, self.data)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str { self.msg }
}

const PUZZLE: &'static str = r"I Heard You Like Registers";
const INPUT: &'static str = r"
gug dec 188 if zpw >= 8
pnt inc 428 if xek < 10
qt inc 274 if d == -8
wv dec 661 if gug > -1
kw dec 518 if zpw > -8
kw inc -894 if vk <= 6
pnt dec 503 if vk == 8
kw dec 644 if fs == 0
vk inc 880 if pnt == 428
sg inc 242 if kmh <= 9
gug inc -853 if kmh < 3
d dec 518 if oa < 1
u dec -948 if ar == 7
qt dec 871 if pnt < 433
qt dec -875 if kw < -2065
d inc -644 if kw == -2056
j dec 521 if qnf == 9
yl inc 705 if pu < 2
pnt dec 150 if d < -1156
fs inc 731 if afx >= -7
fwk inc -417 if qt != -874
ll dec -677 if on == 0
fs inc 249 if vk != 872
on dec 125 if vk < 882
gug dec -805 if vk != 880
kw dec 104 if kw == -2056
on dec 476 if pnt == 278
g dec 939 if bl == 0
qt inc 245 if afx < 6
wv dec -529 if u > 4
on inc -751 if j != 5
qt dec 662 if fs > 977
afx dec 861 if yl == 705
qt dec -110 if rre > -7
d dec 199 if qnf != 3
xek dec 167 if j >= -4
fwk dec 641 if oa > -1
gug dec -799 if kw < -2150
on dec -445 if bl >= -1
fs inc 822 if pnt > 274
fwk dec -493 if j >= 10
pnt inc 315 if vk <= 882
zpw dec -166 if qt >= -1178
fs dec -47 if j > 7
wv dec -325 if oa > -3
kw dec -641 if on < -899
pu inc 911 if pnt <= 594
bl dec -2 if xek > -162
on inc -922 if d < -1354
u dec -400 if vk >= 875
sg dec -376 if kw < -1518
afx inc -22 if kw != -1519
zpw dec -788 if pu != 910
zpw dec -847 if oa == 0
yl inc -418 if kmh >= -1
ar dec -279 if g >= -946
kw dec -472 if vk < 885
g dec 766 if pnt <= 583
wv inc 159 if on >= -1823
d dec 140 if on == -1819
vk dec -160 if rre != 0
afx inc 748 if rre <= 8
u dec 78 if qt > -1181
d inc -970 if g == -939
j dec -91 if zpw != 1801
afx inc 141 if u >= 332
fs inc -564 if qnf > -6
bl inc -937 if on != -1834
ll dec 146 if pu < 919
ar inc -307 if d < -2325
sg inc 370 if xek < -168
ll inc 314 if bl <= -942
sg inc -537 if qt >= -1183
zpw inc 435 if vk < 877
bl dec 725 if g != -939
yl inc 493 if pu != 918
u dec 976 if kmh >= -1
pnt dec 150 if ar < -22
vk dec 734 if g >= -944
wv inc 270 if sg != 90
afx dec -764 if sg >= 82
rre dec 694 if zpw != 1796
fwk inc -885 if kw > -1049
bl inc -567 if afx < -106
on dec 195 if xek >= -168
wv dec 393 if vk <= 154
bl dec -517 if ar == -36
kw inc 669 if wv >= -453
u inc -220 if fwk > -1948
pnt dec -624 if kw < -1049
kw inc -820 if g != -937
ar inc -91 if zpw != 1811
bl dec 490 if qt == -1178
vk dec 599 if j < -2
u inc 719 if qnf >= -8
on inc -24 if ar > -126
bl inc 634 if sg >= 73
j inc -256 if kw < -1859
pu inc -656 if d == -2331
gug inc 295 if afx != -107
bl inc 228 if afx < -109
d dec -300 if ar <= -117
qt dec -749 if qnf > 7
d dec -369 if sg > 75
g dec -839 if wv != -460
xek inc -48 if fs <= 1241
pu inc 273 if yl > 773
ll dec -317 if g < -99
qnf dec 687 if on > -2049
bl inc 763 if vk == 146
fwk inc -925 if gug <= 245
zpw inc 994 if oa <= 4
pnt dec 697 if ar <= -120
fwk dec 277 if fs <= 1242
on dec -508 if pu >= 527
ar dec -261 if qt == -1178
yl dec -566 if qnf >= -691
pu inc -867 if j < -247
yl dec 653 if rre >= -693
oa dec 258 if kmh < 8
xek inc -388 if fs > 1242
g inc 11 if d == -1662
gug inc 512 if bl < -369
kw inc 456 if ar != 144
gug dec 540 if bl >= -375
oa inc -447 if fwk >= -3154
g inc -100 if pu >= -346
g dec 175 if vk != 146
yl inc 31 if fs <= 1244
kmh dec 508 if d >= -1653
rre dec -197 if xek >= -221
j inc 258 if bl > -377
g dec 574 if kw >= -1415
pu dec -514 if j != 3
j dec 168 if vk <= 141
wv inc 964 if zpw < 2802
zpw dec 541 if pu <= 180
ar inc 233 if kw < -1410
g dec 445 if fwk == -3145
vk dec 537 if pnt < 445
gug inc 578 if qt < -1175
rre inc -539 if j <= -2
afx inc -12 if qnf > -695
vk dec -881 if g >= -1209
sg inc 936 if pnt < 448
afx inc -386 if xek > -216
g inc 325 if afx == -511
pu dec -26 if j <= -4
zpw dec 308 if ll < 841
xek dec 62 if qnf != -681
ar dec -27 if pu > 180
fwk inc -863 if pnt >= 438
on dec 256 if sg == 1017
qnf dec -563 if ar != 383
wv dec -970 if afx <= -507
vk dec -15 if sg < 1023
ar inc 398 if ll < 858
ar dec 630 if u > -157
oa dec 465 if fwk > -4012
zpw dec 554 if fs >= 1229
g dec -548 if d <= -1654
oa inc 719 if kw >= -1416
pu inc -429 if qt <= -1172
qnf dec -183 if on == -1796
yl inc -46 if u <= -162
qt dec 19 if ll < 855
sg inc 476 if vk != 502
wv dec 204 if qt >= -1201
ar inc -332 if u != -161
wv dec -656 if oa < -447
on inc -866 if rre <= -491
on inc -615 if kw >= -1416
xek inc 641 if xek >= -285
qt dec -204 if yl != 1385
xek dec 328 if on <= -3275
d inc -504 if ar != -180
qt dec 801 if yl > 1376
fs dec 124 if d != -2176
u dec -602 if zpw <= 1709
fwk dec 845 if pnt == 443
j inc -110 if kw != -1409
u dec -19 if pu != -263
g dec -859 if ll == 848
zpw dec -132 if zpw != 1695
g inc -94 if on != -3268
qt inc -654 if gug != 279
pu inc -111 if zpw >= 1825
fs dec 252 if pnt > 436
gug dec 933 if gug != 280
pu dec 899 if fwk > -4857
fs dec 163 if gug <= -648
pnt dec 829 if bl == -369
bl inc -490 if ll > 844
zpw inc -46 if sg >= 1486
fs dec -121 if qnf > 54
fs inc -516 if gug == -654
qnf dec -799 if fwk > -4858
fs dec -431 if ll == 856
oa dec 159 if fs >= 304
bl inc 713 if kmh != -9
oa inc 933 if pu > -1266
on inc -394 if yl >= 1376
zpw inc -920 if fs == 304
xek inc -573 if on == -3677
xek dec -909 if sg == 1493
g dec 918 if fs < 304
pu dec -502 if ll > 844
vk dec -401 if qt > -1800
sg inc 729 if g >= 427
oa inc -468 if yl != 1387
kw inc 879 if xek >= 937
ll inc -121 if pu < -756
oa inc -868 if pu > -767
sg inc 392 if pu > -770
on dec 335 if ll <= 736
oa inc 394 if afx == -511
bl inc 893 if ar < -187
fs dec 592 if yl != 1377
j dec -53 if kmh < -5
d dec 432 if pnt > -396
fs inc 175 if j == -116
on inc 750 if kw >= -526
fwk dec 821 if qnf != 863
fs inc -185 if pnt == -386
kw dec 914 if kmh > -2
kmh inc 482 if afx != -502
zpw dec 281 if pu >= -753
pu inc 774 if wv < 1919
d dec 549 if g == 430
bl inc 259 if yl > 1374
d dec -439 if d <= -3143
rre dec 79 if g == 430
pnt inc -630 if kw <= -1439
wv dec -137 if on < -3997
zpw dec -622 if pu < -756
kmh inc 87 if j >= -116
xek inc -772 if pu <= -764
j inc -696 if ll > 735
sg dec 655 if vk == 906
ar dec 368 if bl != 1006
vk inc -130 if u >= 457
kw inc -240 if oa > -618
gug inc 880 if gug > -655
qnf dec 985 if vk == 783
j dec 817 if qnf >= 859
ar dec 4 if ar < -181
qnf dec 958 if on != -4008
vk inc -15 if qt < -1786
pnt dec -712 if on != -4011
u dec -326 if sg > 1949
kw inc -144 if oa == -619
zpw dec 690 if gug == 226
kmh dec -942 if on < -4011
fwk dec 357 if zpw > 800
wv dec 423 if qnf < -96
j inc 504 if on > -4012
rre inc -947 if fs <= 120
u inc 160 if u < 788
g dec -222 if d <= -2707
pu inc -994 if xek == 955
qnf dec 621 if xek != 953
qnf inc 991 if fs == 119
gug inc -123 if yl == 1378
gug inc 950 if yl < 1385
g inc -842 if kmh >= 565
sg inc 488 if oa <= -612
sg inc 347 if qnf > 277
ll inc -943 if u < 797
g dec 920 if rre != -1523
afx dec 471 if on <= -4007
wv dec -862 if u == 797
g inc -491 if pu != -766
wv inc -290 if fs > 117
ll inc -630 if pu != -771
bl dec 232 if g != -673
bl inc 21 if ll >= -836
kmh inc -816 if ll != -851
kmh dec -723 if zpw != 796
fwk dec 776 if d >= -2710
qnf dec -723 if sg < 2450
fs inc -276 if zpw >= 794
rre inc -748 if on != -4011
oa inc -81 if fwk >= -6457
xek inc -704 if on < -4003
sg dec 399 if qnf < 988
vk inc -557 if afx < -502
xek inc 378 if j <= 403
sg inc 509 if gug <= 1176
on dec -745 if d == -2708
g dec -323 if rre < -2271
qt inc 835 if fs <= -148
on dec 7 if qnf < 1000
kmh inc -818 if rre < -2266
gug inc 124 if rre >= -2272
kmh inc -844 if qnf <= 991
rre dec 618 if xek < 618
rre inc -159 if ar <= -187
wv dec 433 if pu > -767
afx inc -104 if afx != -510
kmh inc 322 if j >= 396
xek inc -641 if kmh != -29
on inc -721 if ar <= -190
g inc 835 if ll != -848
fs dec 431 if bl <= 767
ll inc -625 if kmh < -17
kmh dec -925 if j == 396
j dec 667 if g != 152
sg inc 289 if fwk > -6442
rre inc 522 if qnf > 992
pu dec -551 if zpw <= 805
kw dec 644 if pnt != -301
ar dec 93 if vk >= 198
qnf dec -243 if qt < -951
sg dec -148 if d >= -2713
qt dec -243 if on > -3981
kw dec -926 if qt <= -954
u inc -874 if pu <= -221
u inc 557 if vk <= 212
g inc -981 if fwk > -6455
kw dec -473 if qt > -952
rre inc -470 if on > -3981
qt inc -147 if g <= -822
fwk dec 185 if fwk <= -6442
rre inc -372 if vk > 194
afx dec -136 if oa != -700
pu inc -921 if j < -265
g inc 615 if yl <= 1377
j inc -592 if j < -267
fwk inc -25 if yl >= 1372
oa dec 47 if qt != -1108
fwk inc 524 if g >= -206
ll inc 38 if ll != -1478
fwk dec 433 if g <= -211
sg inc -541 if d >= -2701
vk dec 222 if zpw == 798
qt inc 683 if vk > -19
wv inc -359 if u <= 1349
xek dec 547 if yl == 1381
qnf inc 933 if wv != 566
on inc -103 if j <= -869
sg inc 765 if fwk > -7087
u inc -731 if pu >= -1133
qt inc -535 if j != -867
yl inc -97 if yl < 1378
g dec -128 if wv > 557
zpw inc -159 if d > -2713
oa dec 558 if kmh > 911
kmh inc -425 if kmh != 906
vk inc -199 if u != 620
fwk dec 134 if d < -2699
sg inc -660 if xek < -14
gug inc -881 if xek >= -31
kw inc 664 if on >= -3998
ll dec -723 if d < -2705
pu inc -376 if kw != -644
qnf dec -798 if zpw > 632
kw dec -603 if qt == -958
xek inc 135 if kw >= -42
kw dec 890 if d == -2713
vk inc -74 if pnt < -303
wv dec -451 if fwk > -7237
afx dec -848 if kmh == 480
rre inc -70 if on != -3989
yl dec 336 if qt != -960
ll dec 16 if zpw > 634
ll dec 644 if kw >= -43
yl dec 692 if gug <= 420
kw dec -118 if kw <= -43
j dec -583 if u < 622
ar dec 957 if j >= -277
qnf dec 313 if g <= -75
gug dec 662 if vk < -297
g dec -45 if oa >= -750
pu dec -99 if qnf != 2654
ll dec 183 if j > -287
u inc 138 if qt == -967
gug inc -667 if vk > -293
j inc -154 if j != -272
afx dec 573 if on > -3991
qnf inc 357 if vk > -298
d dec 500 if afx != -332
g dec -599 if d <= -3203
zpw inc -443 if gug > -249
bl inc 771 if pu > -1137
sg inc 215 if xek == 113
vk inc -827 if ll == -1551
bl inc 220 if xek >= 113
ll inc 981 if ar == -286
gug dec -292 if kw < -33
ll dec 483 if qt <= -968
fwk inc 804 if u >= 609
fwk inc 596 if qt != -956
pnt dec -543 if rre == -2280
pnt dec 362 if rre < -2280
sg dec 118 if d != -3206
vk inc 527 if xek >= 121
sg inc -49 if xek >= 111
j dec -270 if d <= -3210
sg dec 704 if d <= -3199
kmh dec -82 if xek == 113
d dec -217 if sg != 1787
u inc 711 if afx <= -335
j inc 694 if xek > 110
vk dec 745 if j >= 252
fwk dec -192 if afx > -342
d dec -441 if oa >= -751
fwk inc -312 if zpw > 199
d inc 925 if fwk <= -5630
ar inc -500 if vk > -1034
bl dec -411 if wv < 1014
bl dec 281 if j != 267
pu dec 992 if fwk != -5635
ar inc -630 if sg >= 1780
fs dec -580 if pu <= -1124
fs inc -755 if bl != 1900
bl inc 294 if rre < -2273
pnt dec -780 if qnf < 3016
rre dec -608 if fs >= -328
kw dec -637 if vk <= -1027
oa inc -247 if bl != 2185
vk dec 27 if gug == 44
qt dec 184 if bl > 2187
pu inc -867 if wv < 1015
oa dec 650 if qnf >= 3007
kw inc 694 if pu < -1989
fs inc 142 if pu == -1999
pu dec 344 if kmh == 562
rre inc -494 if sg < 1796
yl inc 579 if oa != -1654
pu inc -563 if ar == -916
wv inc -216 if qnf >= 3007
pu dec 692 if vk == -1063
fs inc -790 if zpw != 189
rre dec 705 if xek < 106
d dec 878 if u <= 1331
bl inc -666 if qnf != 3011
kw inc -780 if sg != 1785
vk dec -633 if bl >= 2182
xek dec 302 if wv < 801
ar dec 867 if afx != -348
on dec -997 if pnt != 1027
fs inc 915 if g != 552
on inc -139 if sg >= 1779
wv inc -719 if oa < -1640
qnf inc 140 if u != 1323
bl inc 882 if yl == 831
on dec 398 if oa <= -1646
wv dec 176 if pu == -3598
sg inc 824 if oa >= -1652
g inc -445 if pnt != 1023
oa dec -63 if fs > -75
xek inc 557 if pu <= -3597
afx inc 531 if xek >= 360
zpw dec -837 if u >= 1335
gug inc -955 if kw > 508
fs inc 600 if ar > -1784
yl inc -548 if kmh < 566
afx dec -66 if fwk < -5634
ar dec -109 if pnt < 1023
kmh dec -892 if d >= -2512
fs dec -434 if ll < -570
d dec -491 if d == -2503
bl dec -558 if oa < -1574
ar inc 719 if j < 263
on dec 44 if kw >= 506
d dec 988 if kmh <= 1460
zpw dec 253 if oa > -1589
oa inc 325 if u >= 1320
vk inc -270 if gug == -920
rre dec 52 if j != 262
fs dec 922 if fwk != -5629
g dec 711 if gug < -917
bl dec 255 if g < 125
qnf inc 119 if fs == 47
afx inc -108 if qnf != 3262
afx inc 10 if vk == -430
pu inc -928 if wv == -101
zpw inc 787 if zpw >= -66
sg inc -484 if xek == 368
afx dec -402 if kw != 506
j inc -256 if oa == -1256
rre inc -305 if sg == 2128
fwk dec -111 if u != 1320
kmh inc 359 if wv == -101
pu dec 806 if gug <= -906
gug dec 885 if ar > -961
rre inc 443 if afx != 561
qt inc 353 if pnt < 1022
vk dec -731 if ll != -577
ll inc -816 if j > -3
pu inc -807 if qt > -795
yl dec 805 if qnf == 3270
fs dec 419 if kmh == 1809
ar inc -100 if fwk > -5525
sg inc -111 if rre >= -3123
rre dec 452 if g < 125
ll dec -832 if gug > -1790
wv dec -695 if g != 124
fwk inc -373 if wv > 587
zpw inc -418 if g < 124
d inc -903 if j > 2
yl dec 115 if j != 13
ar inc -962 if fs <= 49
d inc -367 if kw <= 517
u dec 584 if qt < -783
on inc 69 if bl >= 3374
d dec -647 if vk < 308
xek dec 174 if kmh > 1803
oa inc 313 if kmh == 1813
qnf inc -309 if fs < 53
fs dec -554 if qt == -789
kmh dec 776 if fs == 601
wv dec -742 if kw == 515
fwk inc -414 if wv == 589
fs inc 504 if xek >= 187
kw dec 576 if j < 6
ll inc 639 if qnf <= 2964
oa inc 322 if fs != 1105
ll inc -533 if kw == -66
afx inc 864 if kmh >= 1029
qt inc 341 if kw <= -66
on dec 20 if bl < 3381
g inc 844 if fwk <= -5890
pu inc -226 if pu > -6145
kmh inc 633 if on == -3126
gug inc 803 if vk > 304
gug inc -625 if u >= 745
xek dec 292 if qnf >= 2955
d dec -480 if on == -3126
ll dec -259 if xek != -89
g inc 552 if zpw >= 308
vk dec 607 if yl == -637
qt inc 938 if fs < 1110
rre dec -813 if ll < -1016
oa dec -723 if rre < -2771
kw inc 376 if bl == 3374
pu dec 186 if on == -3126
fs inc 231 if fwk < -5894
rre dec 402 if sg != 2128
j dec -236 if fs < 1339
pu inc 540 if ar > -2022
rre inc -314 if fwk < -5900
u inc -563 if kw <= 317
afx dec 150 if on != -3127
ll dec -775 if qt <= 494
qt inc 707 if zpw < 315
bl dec 695 if fs == 1344
rre inc -943 if zpw > 305
j inc 275 if fwk < -5887
zpw inc 446 if fs < 1328
g dec -773 if zpw >= 309
on dec -376 if ll > -251
xek inc -406 if d == -3143
rre dec 500 if xek != -501
qt inc 875 if bl > 3366
pu inc 178 if bl >= 3373
gug inc -469 if afx != 1283
kmh inc -439 if bl < 3376
j dec -979 if oa <= -953
kw inc -824 if sg == 2128
pu inc 849 if d >= -3145
kw dec -654 if g == 2283
u dec -192 if j != 517
vk inc -540 if pu > -4993
d dec 474 if on >= -2754
rre dec 435 if sg > 2119
qnf inc 737 if on != -2750
wv inc -940 if oa < -945
oa inc 569 if fs >= 1329
ar dec -851 if kw < -509
kw dec 352 if d != -3619
on dec -301 if yl == -637
oa dec -520 if pnt > 1013
kw inc -914 if wv > 597
zpw inc -947 if sg != 2134
oa inc 703 if gug >= -2888
kmh inc -112 if u < 381
ar dec -100 if qnf > 2963
afx inc 919 if j == 515
u dec 15 if j > 514
sg inc 425 if xek == -504
afx dec -342 if j == 515
sg dec -20 if yl == -637
zpw dec -678 if pu != -4976
yl dec 212 if on < -2445
vk inc 140 if on != -2448
wv dec 850 if xek <= -495
j inc -526 if ll <= -245
zpw dec 790 if kmh <= 1125
qt dec -342 if afx == 2536
rre inc 137 if pnt != 1028
yl inc -548 if pu >= -4984
pnt inc 86 if fs > 1328
yl dec -253 if ll == -248
sg dec -276 if yl < -1145
pu dec -443 if pu >= -4986
afx dec -163 if fwk != -5902
xek inc -760 if vk > -711
wv inc 656 if kmh == 1119
yl dec 619 if u != 364
rre dec 203 if j >= -6
fs inc 319 if rre != -4502
kmh inc 394 if ar == -1166
qnf dec 263 if oa <= 155
pnt dec -670 if oa != 150
u dec -22 if pu >= -4549
yl inc -444 if kw != -863
fs inc 398 if qnf == 2698
fs dec -88 if qnf > 2689
kw inc 211 if rre > -4504
ll inc -46 if ll != -239
qnf inc 887 if ar != -1170
kmh dec 231 if sg != 2573
g dec 769 if yl >= -2212
ll dec -426 if qt > 2407
j dec 232 if kmh != 1520
g dec 796 if kmh != 1511
g inc 151 if g < 727
afx inc 910 if wv > 405
pu inc -847 if pu == -4549
ar dec 920 if u > 377
gug inc 648 if j > -248
fwk dec 603 if rre >= -4513
u inc -355 if zpw < -745
qnf dec -820 if d == -3617
on inc -85 if on >= -2452
yl inc -967 if d >= -3615
u dec -679 if d <= -3613
ar inc 199 if g >= 862
sg inc -148 if fs > 2140
kmh dec -747 if j < -241
kmh inc -598 if wv == 400
gug inc 783 if oa != 156
qt inc 234 if ll != 142
pnt dec -408 if bl >= 3374
on dec 978 if fwk <= -6498
fs dec 516 if zpw > -753
sg dec 968 if d >= -3626
wv inc 43 if g <= 873
j inc -335 if fs >= 1619
rre dec -388 if yl == -2207
fwk dec 363 if u > 699
zpw inc 9 if d != -3616
sg inc 925 if on > -3516
qt dec 603 if kmh > 1653
oa inc -402 if wv >= 440
g inc 906 if fs >= 1621
pu inc 876 if oa < -246
pnt inc -871 if kw <= -861
vk dec 538 if kw < -856
ll dec -831 if kw == -866
pnt inc 418 if d <= -3615
j inc -308 if gug == -1459
vk inc 574 if ll <= 969
zpw dec -733 if kmh > 1656
fwk dec -349 if fs == 1625
u dec -761 if bl < 3378
oa dec 115 if pu >= -3667
pnt dec 50 if vk > -674
qnf inc 663 if oa != -372
rre inc 383 if kmh <= 1670
fwk dec 126 if pnt > 1670
wv dec -199 if ar > -1893
ar inc -385 if pnt < 1674
d dec -814 if rre > -3745
wv inc 849 if j >= -888
pnt inc 934 if wv <= 1484
kmh inc -508 if sg < 2376
kmh inc 664 if yl != -2198
qt inc -409 if sg < 2385
gug inc 953 if u == 1466
yl inc -793 if oa != -366
xek dec 245 if ar < -1892
oa dec 942 if u == 1466
yl inc 935 if zpw <= 1
ar dec 91 if fwk < -6644
u dec -210 if kw > -864
sg dec -231 if pnt < 1683
pnt inc 825 if qnf == 5068
j inc -859 if vk > -671
ar dec -528 if kw <= -876
rre inc 120 if vk < -662
kw inc 150 if d == -2803
ll dec -984 if gug > -514
vk dec 415 if xek <= -1257
vk inc 813 if gug <= -501
on dec 866 if qt < 1633
bl inc -270 if qt < 1644
j inc -814 if bl != 3104
sg dec -856 if rre == -3620
gug inc 760 if oa > -1304
ll inc -982 if zpw <= -2
g inc -433 if oa < -1310
u dec 487 if g == 1343
pu dec -898 if ar < -1885
fwk dec 121 if u > 970
afx inc 790 if qnf < 5069
kmh dec -820 if d > -2805
pu dec 443 if pnt > 2502
on dec -777 if afx < 3498
fwk inc -99 if kmh != 3137
ll dec -997 if vk <= -264
qt inc -696 if gug > -512
fs dec 426 if j < -1741
yl inc -897 if ll < 1968
sg dec 350 if zpw >= -10
yl dec 124 if d != -2811
fwk dec -653 if pnt <= 2507
kw dec -737 if ll > 1956
sg inc 574 if u <= 983
u dec 942 if kmh < 3156
ll dec 604 if yl == -3086
u inc -995 if rre != -3610
sg inc 27 if kw >= 17
pu inc 586 if ll != 1349
pnt inc 400 if d != -2811
gug inc 660 if ar > -1890
bl inc -297 if afx >= 3491
ar inc 902 if wv > 1486
wv dec 158 if pnt != 2910
kw inc 755 if xek == -1266
g dec -66 if on > -2736
bl inc -905 if on != -2733
gug dec -50 if fwk >= -6211
rre inc -757 if g != 1399
ar dec 121 if wv > 1326
bl dec 155 if ll > 1356
pu dec -398 if ar <= -1113
gug inc -23 if d < -2801
sg inc -283 if oa != -1318
wv inc 959 if pnt != 2904
j dec 364 if ll >= 1357
on dec 195 if oa < -1310
xek dec 884 if vk <= -270
zpw inc 899 if d > -2810
kw inc -398 if fs > 1192
kmh dec 521 if pnt >= 2896
vk dec -937 if xek < -2142
pu inc 586 if qt < 946
qt inc -974 if sg >= 3434
fwk dec 236 if qt != -33
j inc 885 if vk <= 670
g inc 933 if vk <= 665
d inc 607 if wv == 2292
xek dec -823 if pu > -2034
sg inc 916 if ar < -1101
pu inc -983 if pu == -2038
oa dec 25 if ll == 1358
g dec 444 if zpw <= 899
rre inc 892 if g > 1890
vk inc -108 if vk != 665
zpw dec 506 if oa > -1345
yl inc 0 if zpw <= 396
pu dec 311 if xek < -2142
oa inc -148 if rre >= -3491
pnt inc -740 if sg <= 4344
qnf inc -632 if gug > 185
wv inc -947 if kmh < 2620
oa inc -741 if ar <= -1103
wv inc -234 if kmh == 2625
on inc -182 if kw == -377
u dec -98 if qnf <= 5070
g inc 553 if pnt >= 2904
u inc -717 if kw != -371
afx inc 174 if gug < 185
xek inc -922 if pnt < 2913
qnf inc -337 if oa >= -2235
xek dec 837 if fwk < -6439
gug inc -645 if gug < 190
ll dec 388 if xek != -3913
yl dec 707 if fs > 1206
bl inc -743 if kmh >= 2621
pnt dec 888 if j == -1234
ll dec 183 if pu < -3327
d inc 152 if xek > -3909
u dec -752 if rre >= -3479
j inc -436 if fs >= 1193
rre inc 450 if yl < -3086
kw inc 608 if pnt >= 2908
rre inc 761 if qnf >= 4736
gug dec -976 if qt >= -41
afx inc -24 if yl != -3091
qnf dec 827 if rre >= -3488
vk dec -778 if g > 2443
oa dec -590 if bl != 1310
bl dec 240 if vk >= 1449
qt dec 584 if j == -1669
wv dec -260 if j < -1651
fwk dec 280 if qnf != 3903
sg dec 174 if rre != -3487
vk dec 990 if wv <= 2322
oa dec 118 if ll == 787
fwk inc 754 if kmh >= 2625
kmh inc -697 if rre >= -3475
wv inc 21 if pu >= -3336
kmh inc -694 if qnf < 3907
bl dec -383 if qt <= -25
xek dec -201 if bl > 1681
rre inc -606 if kw != -383
qt inc -476 if g == 2451
kw dec 552 if wv != 2341
ar inc -991 if kw < -935
sg inc -405 if on != -3118
pu dec -239 if ll == 787
j dec 515 if gug < 510
rre inc 133 if vk < 459
qt inc 968 if qnf <= 3912
wv dec -131 if kmh > 1929
fs dec 482 if wv < 2474
rre dec -438 if j >= -1654
kmh inc 557 if u != -1577
fwk inc -234 if yl != -3076
vk inc 670 if j < -1652
oa dec 992 if kw == -929
on dec -42 if gug < 518
j inc 53 if wv < 2478
gug dec 996 if ar == -1112
zpw dec 844 if fs == 717
kw dec -785 if g <= 2443
u inc -929 if pnt <= 2907
yl inc 956 if fwk == -6203
gug dec -469 if fs != 722
qnf dec 584 if d > -2052
fwk dec 652 if wv > 2460
kw inc -115 if sg < 3782
qnf dec -866 if ll == 793
oa dec -10 if d == -2044
g dec 533 if ll == 787
bl inc -924 if rre == -3958
xek inc 53 if u == -2506
wv dec -950 if g < 1926
bl inc 83 if gug != 974
oa dec 582 if vk < 1129
u inc -664 if gug <= 990
rre inc 484 if ar < -1109
ll inc 531 if fs > 715
qt inc 642 if fwk == -6855
g dec -504 if bl < 843
j dec -422 if rre < -3961
vk inc -898 if qt > 1093
xek inc -28 if vk <= 230
zpw dec 113 if kw > -1046
on dec -182 if qt != 1095
oa inc -986 if ll == 1318
on dec 660 if afx == 3639
ar dec -912 if kw >= -1049
yl inc -375 if kmh != 1931
kmh inc 913 if xek != -3685
u dec 384 if bl < 836
on inc -151 if xek < -3675
qt inc -363 if fwk != -6857
kw inc 87 if pu == -3085
sg inc 598 if pnt < 2911
yl dec 839 if kw < -1042
oa inc -706 if kmh <= 2841
afx dec -488 if wv == 3420
vk dec -845 if qnf == 3320
fwk dec -658 if sg <= 4374
vk inc -490 if sg < 4375
qt inc -973 if ar != -189
qnf inc -395 if ar <= -194
pnt inc -224 if oa != -4300
d dec 926 if qt <= -233
on inc -776 if qnf >= 2921
fwk dec 249 if yl >= -2976
sg inc 406 if wv != 3424
vk dec 55 if g > 1914
fwk dec 919 if yl != -2977
u dec -752 if zpw > -579
xek dec -89 if afx != 4123
oa dec 633 if g <= 1924
xek dec -332 if j < -1606
gug dec 623 if u != -2428
zpw dec 618 if pnt > 2689
u dec 568 if pu < -3089
yl dec -59 if j != -1603
kmh dec 846 if sg == 4778
pnt dec 897 if g >= 1914
bl inc -753 if qnf < 2918
yl inc 237 if wv < 3417
yl dec 429 if ar > -202
oa dec 381 if pnt >= 1781
kw dec -876 if fwk == -7372
qnf inc 454 if kw == -1044
fs inc 338 if g == 1918
j inc -526 if j != -1607
vk inc 72 if qnf >= 3374
oa dec 674 if wv > 3412
pnt inc -383 if zpw != -569
yl dec 715 if d >= -2969
on inc 292 if wv > 3413
kmh dec 867 if sg != 4778
fs inc -460 if bl < 834
zpw inc -236 if kw <= -1043
sg dec 791 if zpw <= -796
qnf inc 304 if pnt < 1790
vk dec -800 if zpw == -805
rre inc -16 if u == -2986
gug inc 180 if kmh > 1990
bl dec 596 if d <= -2967
u inc 372 if fs == 1055
fs inc -618 if fwk >= -7369
rre inc 610 if on <= -4186
d dec 264 if ll <= 1326
zpw inc 933 if kw <= -1045
qnf inc 780 if kmh < 1997
d dec -354 if bl != 247
vk inc -338 if oa != -5999
ar inc -8 if rre != -3964
kmh inc -480 if j <= -1605
vk inc -568 if ll != 1319
qnf inc 155 if vk < 499
zpw inc -127 if u == -2614
pu inc -731 if vk == 491
vk dec 980 if zpw > -942
fwk dec 888 if zpw >= -938
wv dec -172 if j > -1610
yl dec 91 if on < -4177
bl inc -622 if ll >= 1328
wv dec 335 if on <= -4193
ll dec -983 if qt == -236
bl inc -749 if gug > 537
d inc -836 if on >= -4174
pnt inc -149 if pu != -3824
sg dec -952 if ll == 2301
rre dec 402 if rre > -3973
yl dec 808 if kw > -1040
oa inc -682 if u > -2618
fs inc -720 if pu <= -3827
qt dec -930 if kw != -1039
oa dec 243 if xek == -3260
kmh inc 773 if u <= -2610
ar dec -887 if fwk <= -8259
vk dec -282 if yl >= -3432
sg dec -532 if kmh > 2284
kmh dec -158 if zpw < -922
gug inc 486 if bl > -502
wv inc -944 if wv <= 3598
pnt dec 526 if afx >= 4120
d dec 732 if yl < -3426
fwk inc -107 if bl >= -502
g dec 943 if bl != -506
ar inc -29 if vk > -209
g dec -123 if vk == -207
qnf dec -981 if fs >= 429
ll dec -293 if fwk > -8365
u dec 929 if bl != -502
vk dec 185 if fwk <= -8357
j dec -26 if kw >= -1049
zpw dec 636 if qt < 697
bl inc -830 if vk == -392
fs dec -576 if ar > -229
qt inc -621 if afx > 4119
ar inc -97 if sg == 5471
zpw inc -998 if ar >= -331
j dec -892 if fwk <= -8354
bl dec 349 if bl != -1338
fwk dec 336 if sg > 5462
g inc -806 if ar >= -323
kmh inc 697 if kmh == 2449
zpw inc 205 if pu >= -3827
wv dec -630 if xek <= -3252
xek inc -226 if fwk < -8693
on dec 914 if on <= -4181
yl dec 943 if ar <= -326
gug inc -620 if kmh == 3146
qt dec 345 if j != -689
vk inc 149 if fwk < -8693
fs dec -238 if ll < 2603
oa inc 96 if pnt < 1259
oa dec 110 if g >= 1095
j inc 390 if yl <= -4364
g dec 324 if bl != -1671
pnt inc 657 if oa >= -6936
g inc -787 if ll == 2591
qnf dec -58 if rre != -3969
xek dec 550 if afx != 4129
pu inc -432 if xek < -4027
afx dec 873 if ll > 2594
rre inc 909 if j > -300
ar inc -19 if afx < 4134
pu inc -94 if vk > -245
gug inc 411 if zpw <= -2360
zpw dec -450 if gug <= 336
fs dec -224 if qnf == 4877
j inc 973 if ll == 2596
fs inc 242 if yl >= -4376
ll inc -142 if pu != -4358
kmh dec -428 if afx >= 4126
vk dec -981 if vk < -234
sg inc -472 if vk >= 734
afx dec -275 if oa >= -6941
wv dec -22 if rre != -3067
sg inc -213 if vk > 733
on inc 245 if fwk <= -8695
on dec -920 if vk >= 731
zpw dec 870 if g != 775
ll inc -401 if on == -3931
zpw dec 249 if on != -3941
g dec -541 if u == -2614
";
