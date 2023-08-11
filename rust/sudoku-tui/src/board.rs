pub struct BoardSquare {
    pub value: u8,
    pub marks: Vec<char>,
    pub original: bool,
    pub is_valid: bool,
}

impl BoardSquare {
    pub fn new(value: u8, original: bool, is_valid: bool) -> Self {
        Self {
            value,
            marks: vec![' '; 9],
            original,
            is_valid,
        }
    }

    pub fn set_value(&mut self, v: u8) {
        if !self.original {
            self.value = v;
            self.marks = vec![' '; 9]; // clear marks
        }
    }

    pub fn set_valid(&mut self, v: bool) {
        self.is_valid = v;
    }

    pub fn toggle_mark(&mut self, d: usize) {
        if self.marks[d - 1] == ' ' {
            self.marks[d - 1] = char::from_digit(d as u32, 10).unwrap();
        } else {
            self.marks[d - 1] = ' ';
        }
    }

    pub fn remove_mark(&mut self, d: usize) {
        if d == 0 {
            return;
        }
        self.marks[d - 1] = ' ';
    }

    pub fn get_marks(&self) -> String {
        format!(
            "{} {} {}\n{} {} {}\n{} {} {}",
            self.marks[0],
            self.marks[1],
            self.marks[2],
            self.marks[3],
            self.marks[4],
            self.marks[5],
            self.marks[6],
            self.marks[7],
            self.marks[8],
        )
    }

    pub fn has_marks(&self) -> bool {
        for &mark in self.marks.iter() {
            if mark != ' ' {
                return true;
            }
        }
        false
    }
}
pub struct Board {
    pub boxes: Vec<Vec<BoardSquare>>,
    pub is_solved: bool,
    pub mark_mode: bool,
}

impl Board {
    pub fn from_bytes(bytes: [u8; 81]) -> Self {
        let mut boxes = vec![];
        for row in 0..9 {
            let mut box_row = vec![];
            for col in 0..9 {
                box_row.push(BoardSquare::new(
                    bytes[row * 9 + col],
                    bytes[row * 9 + col] != 0,
                    true,
                ));
            }
            boxes.push(box_row);
        }
        Self {
            boxes,
            is_solved: false,
            mark_mode: false,
        }
    }

    pub fn set_box(&mut self, row: usize, col: usize, v: u8) {
        self.boxes[row][col].set_value(v);

        // Change marks
        // Change marks in row
        for j in 0..9 {
            self.boxes[row][j].remove_mark(v as usize);
        }
        // Check column
        for i in 0..9 {
            self.boxes[i][col].remove_mark(v as usize);
        }
        // Check sub-box
        let sub_row = (row / 3) * 3;
        let sub_col = (col / 3) * 3;
        for i in sub_row..sub_row + 3 {
            for j in sub_col..sub_col + 3 {
                self.boxes[i][j].remove_mark(v as usize);
            }
        }
    }

    pub fn check_validity(&self, row: usize, col: usize) -> bool {
        let curr_box_value = self.boxes[row][col].value;
        // Check row
        for j in 0..9 {
            if j != col && self.boxes[row][j].value == curr_box_value {
                return false;
            }
        }
        // Check column
        for i in 0..9 {
            if i != row && self.boxes[i][col].value == curr_box_value {
                return false;
            }
        }
        // Check sub-box
        let sub_row = (row / 3) * 3;
        let sub_col = (col / 3) * 3;
        for i in sub_row..sub_row + 3 {
            for j in sub_col..sub_col + 3 {
                if i != row && j != col && self.boxes[i][j].value == curr_box_value {
                    return false;
                }
            }
        }
        true
    }
}
