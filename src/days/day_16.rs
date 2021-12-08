use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let parsed = input.chars().map(|c| c == '1').to_vec();
    let inverted = parsed.iter().rev().map(|b| !*b).to_vec();

    let final_len = 35651584;
    // let final_len = 0x2222200000; // my own upping the ante: this _still_ works O_O

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
    // Option one: Just run the algorithm as intended and store the bits.
    // Takes a lot of memory, but is faster.
    let num_filler_bits = final_len / (parsed.len() + 1) + 1;
    let mut filler_bits = vec![false; num_filler_bits];
    let mut i = 1;

    while i < num_filler_bits {
        let (src, dst) = filler_bits.split_at_mut(i);
        // the added `0` is already in the vec, since it was initialized with `false`.
        src.iter()
            .rev()
            .zip(dst[1..].iter_mut())
            .for_each(|(src, dst)| {
                *dst = !*src;
            });
        i += src.len() + 1;
    }
    let mut filler_bits = filler_bits.into_iter(); // convert to iterator for convenience

    // Option two: Use a generator
    // Takes like 99% less memory, but the runtime is twice as long
    // struct FillerBits {
    //     stack: Vec<(usize, bool, bool)>,
    //     depth: usize,
    //     max_depth: usize,
    //     val: bool,
    //     left: bool,
    // }
    // impl FillerBits {
    //     pub fn new(final_len: usize, segment_len: usize) -> Self {
    //         let max_depth = (final_len as f32 / segment_len as f32).log2().ceil() as usize;
    //         Self {
    //             stack: Vec::with_capacity(max_depth * 2),
    //             depth: 0,
    //             max_depth,
    //             val: false,
    //             left: true,
    //         }
    //     }
    // }
    // impl Iterator for FillerBits {
    //     type Item = bool;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.depth == self.max_depth {
    //             if let Some((next_depth, next_val, prev_val)) = self.stack.pop() {
    //                 self.depth = next_depth;
    //                 self.val = next_val;
    //                 self.left = false;
    //                 Some(prev_val)
    //             } else {
    //                 None
    //             }
    //         } else {
    //             self.depth += 1;
    //             let next_val = self.val == self.left;
    //             self.stack.push((self.depth, !next_val, self.val));
    //             self.left = true;
    //             self.val = next_val;
    //             self.next()
    //         }
    //     }
    // }
    // let mut filler_bits = FillerBits::new(final_len, parsed.len() + 1);
    // end of Option two

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
        print!("{}", current as u8);

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
