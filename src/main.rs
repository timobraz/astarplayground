use pathfinding::prelude::astar;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);
impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1))
    }

    fn successors(&self) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;

        vec![
            Pos(x + 1, y + 2),
            Pos(x + 1, y - 2),
            Pos(x - 1, y + 2),
            Pos(x - 1, y - 2),
            Pos(x + 2, y + 1),
            Pos(x + 2, y - 1),
            Pos(x - 2, y + 1),
            Pos(x - 2, y - 1),
        ]
        .into_iter()
        .map(|p| (p, 1))
        .collect()
    }
}
fn main() {
    let contents = fs::read_to_string("./README2.txt").expect("failed to read");
    let mut numbers = contents.split(' ').map(|num| num.parse::<u32>());
    let mut numcollection: Vec<Vec<u32>> = vec![];
    let mut endstring = String::from("");
    let mut line: Vec<u32> = vec![];
    let mut index = 1;
    let mut nextnum = numbers.next().unwrap();
    while index < 9765625 {
        // println!("{index} ,{:?}", &nextnum);
        if nextnum == Ok(index) {
            // line.push(0);
            nextnum = numbers.next().unwrap();
            if (index == 4296890 || index == 4687515) {
                endstring += "X";
            } else {
                endstring += "█";
            }
        } else {
            endstring += " ";
            // line.push(1);
        }
        if index % 1953125 == 0 {
            endstring += "\r\n";
            // numcollection.push(line.clone());
            // line.clear();
        }
        index += 1;
    }
    endstring += "1";
    println!("writing");
    fs::write("numcollection.txt", endstring).expect("failed to write to file");

    // fs::write("arrays.json", format!("{:?}", numcollection)).expect("failed to write to file");
    // println!("{:?}", &numcollection);
    const GOAL: Pos = Pos(1953125, 3125);
    // let result = astar(
    //     &Pos(1, 1),
    //     |p| p.successors(),
    //     |p| p.distance(&GOAL) / 3,
    //     |p| *p == GOAL,
    // )
    // .expect("failed to find path");
    // dbg!("{:?}", &result.0);
}
