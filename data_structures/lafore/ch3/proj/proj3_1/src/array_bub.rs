pub struct ArrayBub {
    a: Box<[i64]>,
    n_elems: usize,
}

impl ArrayBub {
    pub fn new(size: usize) -> ArrayBub {
        ArrayBub {
            a: vec![0; size].into_boxed_slice(),
            n_elems: 0,
        }
    }

    pub fn insert(&mut self, value: i64) {
        self.a[self.n_elems] = value;
        self.n_elems += 1;
    }

    pub fn display(&self) {
        print!("[");

        if self.n_elems > 0 {
            print!("{}", self.a[0]);

            for i in 1..self.n_elems {
                print!(", {}", self.a[i]);
            }
        }

        println!("]");
    }

    pub fn bubble_sort(&mut self) {
        let mut out_decr = self.n_elems - 1;
        let mut out_incr = 1;

        while out_decr > out_incr {

            for inner in 0..out_decr {
                if self.a[inner] > self.a[inner + 1] {
                    self.a.swap(inner, inner + 1);
                }
            }

            out_decr -= 1;

            for inner in (out_incr..self.n_elems).rev() {
                if self.a[inner] < self.a[inner - 1] {
                    self.a.swap(inner, inner - 1);
                }
            }

            out_incr += 1;
        }
    }
}
