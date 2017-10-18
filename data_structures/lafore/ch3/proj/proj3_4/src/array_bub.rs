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

    pub fn odd_even_sort(&mut self) {
        loop {
            let mut change_flag = false;
            let even_index = 0;
            change_flag = self.process_arr(even_index);
            let odd_index = 1;
            change_flag = self.process_arr(odd_index);

            if !change_flag {
                break;
            }
        }
    }

    fn process_arr(&mut self, mut index: usize) -> bool {
        let mut change_flag = false;

        while index + 1 < self.n_elems {
            if self.a[index] > self.a[index + 1] {
                self.a.swap(index, index + 1);
                change_flag = true;
            }

            index += 2;
        }

        change_flag
    }
}
