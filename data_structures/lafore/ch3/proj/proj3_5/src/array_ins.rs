pub struct ArrayIns {
    a: Box<[i64]>,
    n_elems: usize
}

impl ArrayIns {
    pub fn new(max: usize) -> ArrayIns {
        ArrayIns {
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

    pub fn insertion_sort(&mut self) {
        let mut comp_ctr = 0;
        let mut copy_ctr = 0;

        for out in 1..self.n_elems {
            let temp = self.a[out];
            copy_ctr += 1;
            let mut inner = out;

            while inner > 0 {
                comp_ctr += 1;

                if self.a[inner - 1] >= temp {
                    self.a[inner] = self.a[inner - 1];
                    copy_ctr += 1;
                } else {
                    break;
                }

                inner -= 1;
            }

            self.a[inner] = temp;
            copy_ctr += 1;
        }

        println!("copying amount: {}", copy_ctr);
        println!("compares amount: {}", comp_ctr);
    }
}
