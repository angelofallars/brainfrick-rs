use getch::Getch;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum Error {
    CellOverflow { position: usize },
    CellUnderflow { position: usize },
    PointerOutOfLeftBound,
    PointerOutOfRightBound,
    UnclosedLeftBrace { position: usize },
    StrayRightBrace { position: usize },
    IoError,
}

const CELL_COUNT: usize = 30_000;

struct DataPointer {
    pub address: usize,
}

impl DataPointer {
    fn new() -> DataPointer {
        DataPointer { address: 0 }
    }

    fn increment(&mut self) -> Result<(), Error> {
        if self.address >= CELL_COUNT - 1 {
            return Err(Error::PointerOutOfRightBound);
        }

        self.address += 1;
        Ok(())
    }

    fn decrement(&mut self) -> Result<(), Error> {
        if self.address == 0 {
            return Err(Error::PointerOutOfLeftBound);
        }

        self.address -= 1;
        Ok(())
    }
}

pub fn create_byte_cells() -> [u8; CELL_COUNT] {
    [0; CELL_COUNT]
}

pub fn process_commands(byte_cells: &mut [u8; CELL_COUNT], commands: String) -> Result<(), Error> {
    let mut data_pointer: DataPointer = DataPointer::new();
    let bytes = commands.as_bytes();
    let left_brace_pairs = find_matching_braces(&commands)?;
    let right_brace_pairs = reverse_hash_map(&left_brace_pairs);

    let mut i = 0;
    while i < bytes.len() {
        let current_cell_byte = byte_cells[data_pointer.address];
        let char = char::from_u32(bytes[i].try_into().unwrap()).unwrap();

        match char {
            '>' => {
                data_pointer.increment()?;
            }
            '<' => {
                data_pointer.decrement()?;
            }
            '+' => {
                if current_cell_byte < 255 {
                    byte_cells[data_pointer.address] += 1;
                } else {
                    return Err(Error::CellOverflow {
                        position: data_pointer.address,
                    });
                }
            }
            '-' => {
                if current_cell_byte > 0 {
                    byte_cells[data_pointer.address] -= 1;
                } else {
                    return Err(Error::CellUnderflow {
                        position: data_pointer.address,
                    });
                }
            }
            '.' => {
                let byte = current_cell_byte;
                let byte_char = char::from_u32(byte.try_into().unwrap()).unwrap();
                print!("{}", byte_char);
            }
            ',' => {
                let getch = Getch::new();
                let char = getch.getch();

                match char {
                    Ok(char) => byte_cells[data_pointer.address] = char,
                    Err(_) => {
                        return Err(Error::IoError);
                    }
                }
            }
            '[' => {
                if current_cell_byte == 0 {
                    i = *left_brace_pairs.get(&i).unwrap();
                }
            }
            ']' => {
                if current_cell_byte != 0 {
                    i = *right_brace_pairs.get(&i).unwrap();
                }
            }
            _ => {}
        }

        i += 1;
    }

    print!("\n");

    Ok(())
}

fn find_matching_braces(contents: &String) -> Result<HashMap<usize, usize>, Error> {
    let mut stack: Vec<usize> = Vec::new();

    let chars = contents.as_bytes();

    let mut match_map: HashMap<usize, usize> = HashMap::new();

    for i in 0..chars.len() {
        match chars[i] {
            // [
            91 => stack.push(i),
            // ]
            93 => {
                let left_brace_index = stack.pop();
                if let None = left_brace_index {
                    return Err(Error::StrayRightBrace { position: i });
                }

                let left_brace_index = left_brace_index.unwrap();
                let right_brace_index = i;

                match_map.insert(left_brace_index, right_brace_index);
            }
            _ => {}
        }
    }

    if stack.len() > 0 {
        return Err(Error::UnclosedLeftBrace { position: stack[0] });
    }

    return Ok(match_map);
}

fn reverse_hash_map<T, E>(hash_map: &HashMap<T, E>) -> HashMap<E, T>
where
    T: Eq,
    T: Hash,
    T: Copy,
    E: Eq,
    E: Hash,
    E: Copy,
{
    let mut reversed_map: HashMap<E, T> = HashMap::new();

    for key in hash_map.keys() {
        let value = hash_map.get(key).unwrap();

        reversed_map.insert(*value, *key);
    }

    return reversed_map;
}
