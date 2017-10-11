pub struct ArraySel {
    a: Box<[i64]>,
    n_elems: usize
}

impl ArraySel {
    pub fn new(max: usize) -> ArraySel {
        ArraySel {
            a: vec![0; max].into_boxed_slice(),
            n_elems: 0            
        }
    }

    pub fn insert(&mut self, value: i64) {
        self.a[self.n_elems] = value;
        self.n_elems += 1;
    }

    pub fn display(&self) {
        for i in 0..self.n_elems {
            print!("{} ", self.a[i]);
        }

        println!();
    }

    pub fn selection_sort(&mut self) {
        let mut min;

        for out in 0..self.n_elems - 1 {
            min = out;

            for inner in out + 1..self.n_elems {
                if self.a[inner] < self.a[min] {
                    min = inner;
                }
            }

            self.a.swap(out, min);
        }
    }
}
