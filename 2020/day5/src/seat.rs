pub struct Seat {
    pub identifier: Vec<char>
}

impl Seat {
    pub fn get_seat(&self) -> (i8, i8) {
        let mut min_row = 0;
        let mut max_row = 127;
        let mut min_column = 0;
        let mut max_column = 7;
        for c in self.identifier.iter() {
            match c {
                'F' => {
                    let minus: f32 = (max_row as f32 - min_row as f32) / 2 as f32;
                    max_row -= minus.round() as i8;
                    //println!("Min row and max row: {:?}", (min_row, max_row));
                },
                'B' => {
                    let plus: f32 = (max_row as f32 - min_row as f32) / 2 as f32;
                    min_row += plus.round() as i8;
                    //println!("Min row and max row: {:?}", (min_row, max_row));
                },
                'R' => {
                    let plus: f32 = (max_column as f32 - min_column as f32) / 2 as f32;
                    min_column += plus.round() as i8;
                    //println!("Min column and max column: {:?}", (min_column, max_column));
                },
                'L' => {
                    let minus: f32 = (max_column as f32 - min_column as f32) / 2 as f32;
                    max_column -= minus.round() as i8;
                    //println!("Min column and max column: {:?}", (min_column, max_column));
                }
                _ => ()
            }
        }
        let row;
        let column;
        if self.identifier[6] == 'F' {
            row = min_row;
        } else {
            row = max_row;
        }
        if self.identifier[9] == 'R' {
            column = max_column;
        } else {
            column = min_column;
        }
        return (row, column);
    }

    pub fn get_id(&self) -> i32 {
        let seat = self.get_seat();
        return seat.0 as i32 * 8 + seat.1 as i32;
    }
}
