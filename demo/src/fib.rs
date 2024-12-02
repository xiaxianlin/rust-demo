use std::{mem::swap, ops::Index};

pub struct Recurrence {
    pub mem: [u64; 2],
    pub pos: usize,
}

impl Recurrence {
    pub fn new() -> Self {
        Recurrence {
            mem: [0, 1],
            pos: 0,
        }
    }
}

impl Iterator for Recurrence {
    type Item = u64;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 2 {
            let next_val = self.mem[self.pos];
            self.pos += 1;
            Some(next_val)
        } else {
            let n = self.pos;
            let a = IndexOffset {
                slice: &self.mem,
                offset: n,
            };

            let next_val = a[n - 1] + a[n - 2];
            let mut swap_tmp = next_val;
            for i in (0..2).rev() {
                swap(&mut swap_tmp, &mut self.mem[i]);
            }

            self.pos += 1;
            Some(next_val)
        }
    }
}

struct IndexOffset<'a> {
    slice: &'a [u64; 2],
    offset: usize,
}

impl<'a> Index<usize> for IndexOffset<'a> {
    type Output = u64;

    #[inline(always)]
    fn index<'b>(&'b self, index: usize) -> &'b u64 {
        use std::num::Wrapping;

        let index = Wrapping(index);
        let offset = Wrapping(self.offset);
        let window = Wrapping(2);

        let real_index = index - offset + window;
        &self.slice[real_index.0]
    }
}
