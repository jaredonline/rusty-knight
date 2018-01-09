#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Position {
    pub column: Column,
    pub row: u8,
}

impl Position {
    pub fn new(column: Column, row: u8) -> Position {
        Position {
            column: column,
            row: row,
        }
    }
}

impl From<&'static str> for Position {
    fn from(s: &'static str) -> Position {
        let s = String::from(s);
        s.into()
    }
}

impl From<String> for Position {
    fn from(s: String) -> Position {
        if s.chars().count() != 2 {
            panic!("string based position must be exactly 2 characters");
        }

        let mut chars = s.chars();
        let col = chars.next().unwrap();
        let row = chars.next().unwrap();

        Position {
            column: col.into(),
            row: row.to_digit(10).unwrap() as u8,
        }
    }
}

impl From<usize> for Position {
    fn from(u: usize) -> Position {
        let row = 8 - (u / 8);
        let col = match u % 8 {
            0 => Column::A,
            1 => Column::B,
            2 => Column::C,
            3 => Column::D,
            4 => Column::E,
            5 => Column::F,
            6 => Column::G,
            7 => Column::H,

            _ => unreachable!(),
        };

        Position {
            column: col,
            row: row as u8,
        }
    }
}

impl Into<usize> for Position {
    fn into(self) -> usize {
        let row = (8 - self.row) as usize;
        let col = match self.column {
            Column::A => 0,
            Column::B => 1,
            Column::C => 2,
            Column::D => 3,
            Column::E => 4,
            Column::F => 5,
            Column::G => 6,
            Column::H => 7,
        };

        (row * 8) + col
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<char> for Column {
    fn from(c: char) -> Column {
        match c {
            'a' => Column::A,
            'b' => Column::B,
            'c' => Column::C,
            'd' => Column::D,
            'e' => Column::E,
            'f' => Column::F,
            'g' => Column::G,
            'h' => Column::H,
            _ => panic!(format!("invalid column {} supplied", c)),
        }
    }
}

impl Into<i8> for Column {
    fn into(self) -> i8 {
        match self {
            Column::A => 1,
            Column::B => 2,
            Column::C => 3,
            Column::D => 4,
            Column::E => 5,
            Column::F => 6,
            Column::G => 7,
            Column::H => 8,
        }
    }
}

impl From<i8> for Column {
    fn from(i: i8) -> Column {
        match i {
            1 => Column::A,
            2 => Column::B,
            3 => Column::C,
            4 => Column::D,
            5 => Column::E,
            6 => Column::F,
            7 => Column::G,
            8 => Column::H,
            _ => panic!(format!("invalid column {} supplied", i)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_position() {
        let p: Position = "a1".into();
        assert_eq!(p.column, Column::A);
        assert_eq!(p.row, 1);

        let p: Position = "b2".into();
        assert_eq!(p.column, Column::B);
        assert_eq!(p.row, 2);
    }
}
