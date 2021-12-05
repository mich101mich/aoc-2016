use crate::utils::*;
use std::iter::Peekable;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    fn get_length(iter: &mut dyn Iterator<Item = char>) -> usize {
        let mut length = 0;
        let mut iter = iter.peekable();
        while iter.peek().is_some() {
            length += iter.by_ref().take_while(|c| *c != '(').count();
            if iter.peek().is_none() {
                break;
            }
            let num_chars = parse_u(&iter.by_ref().take_while(|c| *c != 'x').to_string());
            let num_reps = parse_u(&iter.by_ref().take_while(|c| *c != ')').to_string());

            length += num_reps * get_length(&mut iter.by_ref().take(num_chars));
        }
        length
    }

    let mut iter = input.chars().peekable();
    let length = get_length(&mut iter);
    pv!(length);
}
#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let mut iter = input.chars().peekable();
    let mut length = 0;
    while iter.peek().is_some() {
        length += iter.by_ref().take_while(|c| *c != '(').count();
        if iter.peek().is_none() {
            break;
        }
        let num_chars = parse_u(&iter.by_ref().take_while(|c| *c != 'x').to_string());
        let num_reps = parse_u(&iter.by_ref().take_while(|c| *c != ')').to_string());
        length += num_chars * num_reps;

        iter.nth(num_chars - 1).unwrap();
    }
    pv!(length);
}
