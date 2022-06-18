use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let parsed = input.chars().map(|c| c == '1').to_vec();
    let inverted = parsed.iter().rev().map(|b| !*b).to_vec();

    let final_len = 35651584usize; // == 0x2200000usize;
    // let final_len = 0x22222200000usize; // my own upping the ante: you can add as many 2 or 0 and it will just take longer

    // the easy solution: take the code from part one and change `final_len`.
    // but that is boring.
    // and too slow for my tastes. (90ms)
    // This code takes 8ms.
    // So yeah, worth it.

    // Explanation: The task is to do pairwise equality of bits:
    // 11 or 00 => 1 and 10 or 01 => 0
    // and then repeat that as long es there is an even number.
    // This means that the result is a sort of parity bit of blocks with length
    // `block_size`, which is the largest power of two that divides `final_len`.
    // Since parity bits don't care about the order in which the bits are added,
    // we can pre-calculate the parity of repeating segments.
    // The code always starts with the input 'a', followed by 0, followed by the inverted
    // and reversed version of the previous section:
    // `a 0 !a  0  !!a 1 !a`
    // but since the second part is once again inverted and reversed (`!!a`), it means that
    // we just get `a` again.
    // So, the final sequence is:
    // `a ? !a ? a ? !a ? a ? !a ? a ? !a ?`...
    // where ? are filler bits that we calculate below.
    // One segment of `a ? !a ?` shall be called a `chunk`.
    // Since parity doesn't care about order, we can combine all the `a` and `!a` into
    // `chunk_sum`, and handle the filler bits separately.

    // chunk_len: length of a repeating segment
    // = sequence + filler + inverted sequence + filler
    let chunk_len = parsed.len() * 2 + 2;
    let chunk_sum = parsed
        .iter()
        .copied()
        .chain(inverted.iter().copied())
        .reduce(|a, b| a == b)
        .unwrap();

    // The filler bits.
    // The algorithm starts with a 0 between the segments, adds another 0 and flips
    // the first part around and inverts it.
    // Technical name: The regular paper-folding (or dragon curve) sequence. (https://oeis.org/A014707)
    // For easier calculation: https://oeis.org/A038189
    let mut filler_bits = (1usize..).map(|i| (i >> (i.trailing_zeros() + 1)) & 1 == 1);

    // block_size: largest power of two that divides final_len
    let block_size = (1 << final_len.trailing_zeros());
    let num_blocks = final_len / block_size;

    // The last challenge: blocks don't always end at an exact chunk boundary.
    // So we need to manually run through those boundaries and have any remaining
    // parts of a chunk carry over to the next block.
    let mut carry = vec![];
    for _ in 0..num_blocks {
        let mut current = true; // true is a neutral element for the parity operation
        let mut remaining = block_size;

        remaining -= carry.len();
        for b in carry.drain(..) {
            current = current == b;
        }

        let mut full_chunks = remaining / chunk_len; // notice the integer division
        remaining -= full_chunks * chunk_len;

        // If the sum of a chunk is false, every bit would flip `current`.
        // This can be fast-forwarded by checking if that would happen an odd number
        // of times. If chunk_sum is true, do nothing because true is a neutral element.
        if !chunk_sum && full_chunks & 1 == 1 {
            current = !current;
        }

        // Combine all the filler bits, two per chunk.
        for b in filler_bits.by_ref().take(full_chunks * 2) {
            current = current == b;
        }

        // Assemble the boundary chunk into an iterator, take all that is in our block
        // and put the rest into carry.
        let mut iter = parsed
            .iter()
            .copied()
            .chain(filler_bits.next())
            .chain(inverted.iter().copied())
            .chain(filler_bits.next());

        for b in iter.by_ref().take(remaining) {
            current = current == b;
        }
        if current {
            print!("1");
        } else {
            print!("0");
        }

        carry.extend(iter);
    }
    println!();
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let mut parsed = input.chars().map(|c| c == '1').to_vec();

    let final_len = 272;
    while parsed.len() < final_len {
        let mut b = parsed.iter().map(|&b| !b).to_vec();
        b.reverse();

        parsed.push(false);
        parsed.extend(b);
    }
    parsed.truncate(final_len);
    while parsed.len() % 2 == 0 {
        parsed = parsed.chunks(2).map(|v| v[0] == v[1]).to_vec();
    }
    for b in parsed {
        print!("{}", b as u8);
    }
    println!();
}
