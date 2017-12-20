/*!
# Day 1: Inverse Captcha

The night before Christmas, one of Santa's Elves calls you in a panic. "The printer's broken! We
can't print the Naughty or Nice List!" By the time you make it to sub-basement 17, there are only
a few minutes until midnight. "We have a big problem," she says; "there must be almost fifty bugs
in this system, but nothing else can print The List. Stand in this square, quick! There's no time
to explain; if you can convince them to pay you in stars, you'll be able to--" She pulls a lever
and the world goes blurry.

When your eyes can focus again, everything seems a lot more pixelated than before. She must have
sent you inside the computer! You check the system clock: 25 milliseconds until midnight. With that
much time, you should be able to collect all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day millisecond in the
advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one
star. Good luck!

You're standing in a room with "digitization quarantine" written in LEDs along one wall. The only
door is locked, but it includes a small interface. "Restricted Area - Strictly No Digitized Users
Allowed."

It goes on to explain that you may only leave by solving a captcha to prove you're not a human.
Apparently, you only get one millisecond to solve the captcha: too fast for a normal human, but it
feels like hours to you.

The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of all
digits that match the next digit in the list. The list is circular, so the digit after the last
digit is the first digit in the list.

For example:

- 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and
  the third digit (2) matches the fourth digit.
- 1111 produces 4 because each digit (all 1) matches the next.
- 1234 produces 0 because no digit matches the next.
- 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.

What is the solution to your captcha?

## Part Two

You notice a progress bar that jumps to 50% completion. Apparently, the door isn't yet satisfied,
but it did emit a star as encouragement. The instructions change:

Now, instead of considering the next digit, it wants you to consider the digit halfway around the
circular list. That is, if your list contains 10 items, only include a digit in your sum if the
digit 10/2 = 5 steps forward matches it. Fortunately, your list has an even number of elements.

For example:

- 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
- 1221 produces 0, because every comparison is between a 1 and a 2.
- 123425 produces 4, because both 2s match each other, but no other digit has a match.
- 123123 produces 12.
- 12131415 produces 4.

What is the solution to your new captcha?
*/

extern crate aoc;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 1: {}", PUZZLE);

    let sn: Vec<u32> = input.to_str()
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!(":: Answer 1 is {}", inverse_captcha(sn.iter(), 1));
    println!(":: Answer 2 is {}", inverse_captcha(sn.iter(), sn.len() / 2));
}

/// Calculate the captcha result for the printer for day 1.
/// This works for part one (n=1) and part two (n=vec.len()/2).
pub fn inverse_captcha<'a, I>(it: I, n: usize) -> u32
where I: Iterator<Item = &'a u32> + std::clone::Clone {
    let y = it.clone().cycle().skip(n);
    it.zip(y).fold(0, |acc, (x,y)| if x == y { acc + x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_captcha_correct_sum() {
        let tests = vec![
            // vector       1  n/2
            (vec![1,1,2,2], 3, 0),
            (vec![1,2,1,2], 0, 6),
            (vec![1,1,1,1], 4, 4),
            (vec![1,2,3,4], 0, 0),
            (vec![1,1,1,2], 2, 2),
            (vec![9,1,2,1,2,1,2,9], 9, 6),
        ];

        for t in tests {
            assert_eq!(inverse_captcha(t.0.iter(), 1), t.1);
            assert_eq!(inverse_captcha(t.0.iter(), t.0.len()/2), t.2);
        }
    }
}

const PUZZLE: &'static str = "Inverse Captcha";
const INPUT: &'static str = r"
5994521226795838486188872189952551475352929145357284983463678944777228139398117649129843853837124228353689551178129353548331779783742915361343229141538334688254819714813664439268791978215553677772838853328835345484711229767477729948473391228776486456686265114875686536926498634495695692252159373971631543594656954494117149294648876661157534851938933954787612146436571183144494679952452325989212481219139686138139314915852774628718443532415524776642877131763359413822986619312862889689472397776968662148753187767793762654133429349515324333877787925465541588584988827136676376128887819161672467142579261995482731878979284573246533688835226352691122169847832943513758924194232345988726741789247379184319782387757613138742817826316376233443521857881678228694863681971445442663251423184177628977899963919997529468354953548612966699526718649132789922584524556697715133163376463256225181833257692821331665532681288216949451276844419154245423434141834913951854551253339785533395949815115622811565999252555234944554473912359674379862182425695187593452363724591541992766651311175217218144998691121856882973825162368564156726989939993412963536831593196997676992942673571336164535927371229823236937293782396318237879715612956317715187757397815346635454412183198642637577528632393813964514681344162814122588795865169788121655353319233798811796765852443424783552419541481132132344487835757888468196543736833342945718867855493422435511348343711311624399744482832385998592864795271972577548584967433917322296752992127719964453376414665576196829945664941856493768794911984537445227285657716317974649417586528395488789946689914972732288276665356179889783557481819454699354317555417691494844812852232551189751386484638428296871436139489616192954267794441256929783839652519285835238736142997245189363849356454645663151314124885661919451447628964996797247781196891787171648169427894282768776275689124191811751135567692313571663637214298625367655969575699851121381872872875774999172839521617845847358966264291175387374464425566514426499166813392768677233356646752273398541814142523651415521363267414564886379863699323887278761615927993953372779567675
";
