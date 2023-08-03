// use pathfinding::prelude::astar;
use image::{Rgb, RgbImage};
use std::{char::MAX, fs};
type Color = Rgb<u8>;
// type Point = (u32, u32);
const WHITE: Color = Rgb([255, 255, 255]);
const BLACK: Color = Rgb([0, 0, 0]);
const RED: Color = Rgb([255, 0, 0]);
const MAX_NUMBER: u32 = 9765625;
// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Pos(i32, i32);
// impl Pos {
//     fn distance(&self, other: &Pos) -> u32 {
//         (self.0.abs_diff(other.0) + self.1.abs_diff(other.1))
//     }

//     fn successors(&self) -> Vec<(Pos, u32)> {
//         let &Pos(x, y) = self;

//         vec![
//             Pos(x + 1, y + 2),
//             Pos(x + 1, y - 2),
//             Pos(x - 1, y + 2),
//             Pos(x - 1, y - 2),
//             Pos(x + 2, y + 1),
//             Pos(x + 2, y - 1),
//             Pos(x - 2, y + 1),
//             Pos(x - 2, y - 1),
//         ]
//         .into_iter()
//         .map(|p| (p, 1))
//         .collect()
//     }
// }
fn main() {
    let contents = fs::read_to_string("./README2.txt").expect("failed to read");
    let mut numbers = contents.split(' ').map(|num| num.parse::<u32>().unwrap());
    draw_image(1953125, &mut numbers, 25);
    // generate_txt(390625, &mut numbers);
    // println!("{:?}", &numcollection);
    // const GOAL: Pos = Pos(1953125, 3125);
    // let result = astar(
    //     &Pos(1, 1),
    //     |p| p.successors(),
    //     |p| p.distance(&GOAL) / 3,
    //     |p| *p == GOAL,
    // )
    // .expect("failed to find path");
    // dbg!("{:?}", &result.0);
}

fn draw_image<I>(linebreak: u32, mut numbers: I, widthmax: u32)
where
    I: Iterator<Item = u32>,
{
    let important_pixels = vec![
        1, 2, 3, 1953128, 3906253, 3906254, 3906255, 3906260, 3906265,
    ];
    let mut index = 1;
    let mut nextnum = numbers.next().unwrap();
    let mut img = RgbImage::from_pixel(widthmax, MAX_NUMBER / linebreak, BLACK);

    while index < MAX_NUMBER {
        let line = ((index / linebreak) as f64).floor() as u32;
        let xpixel = index - 1;
        if nextnum == index {
            nextnum = numbers.next().unwrap_or(0);
            if xpixel % linebreak > widthmax {
                index += 1;
                continue;
            }
            let xoffset = xpixel % linebreak;
            if let Ok(ind) = important_pixels.binary_search_by(|probe| probe.cmp(&index)) {
                img.put_pixel(
                    xoffset,
                    line,
                    Rgb([
                        (255.0
                            * ((ind as f64 / important_pixels.len() as f64) + 0.5).clamp(0f64, 1.0))
                        .floor() as u8,
                        0,
                        0,
                    ]),
                );
            } else {
                img.put_pixel(xoffset, line, WHITE)
            }
        }
        index += 1;
    }

    let _ = img.save("output.png");
}

fn generate_txt<I>(linebreak: u32, mut numbers: I)
where
    I: Iterator<Item = u32>,
{
    let mut endstring = String::from("");
    let mut index = 1;
    let mut nextnum = numbers.next().unwrap();
    // let mut numcollection: Vec<Vec<u32>> = vec![];
    // let mut line: Vec<u32> = vec![];
    while index < MAX_NUMBER {
        // println!("{index} ,{:?}", &nextnum);
        if nextnum == index {
            // line.push(0);
            nextnum = numbers.next().unwrap();
            if index == 4296890 || index == 4687515 {
                endstring += "X";
            } else {
                endstring += "█";
            }
        } else {
            endstring += " ";
            // line.push(1);
        }
        if index % linebreak == 0 {
            endstring += "\r\n";
            // numcollection.push(line.clone());
            // line.clear();
        }
        index += 1;
    }
    endstring += "1";
    // println!("writing");
    fs::write(format!("{linebreak}.txt"), endstring).expect("failed to write to file");

    // fs::write("arrays.json", format!("{:?}", numcollection)).expect("failed to write to file");
}
