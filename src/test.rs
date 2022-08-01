use crate::interpreter::{create_byte_cells, process_commands, Error};

#[test]
fn test_increment() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from("++++");

    let result = process_commands(&mut byte_cells, snippet);
    assert!(result.is_ok());

    assert!(byte_cells[0] == 4);
}

#[test]
fn test_decrement() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from("++++++++------");

    let result = process_commands(&mut byte_cells, snippet);
    assert!(result.is_ok());

    assert!(byte_cells[0] == 2);
}

#[test]
fn test_move_right() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from(">>++");

    let result = process_commands(&mut byte_cells, snippet);
    assert!(result.is_ok());

    assert!(byte_cells[2] == 2);
}

#[test]
fn test_move_left() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from(">><<++");

    let result = process_commands(&mut byte_cells, snippet);
    assert!(result.is_ok());

    assert!(byte_cells[0] == 2);
}

#[test]
fn test_comments() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from("Some Comments >> HERE <<++");

    let result = process_commands(&mut byte_cells, snippet);
    assert!(result.is_ok());

    assert!(byte_cells[0] == 2);
}

#[test]
fn test_braces() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from("+++>+++++<[->+<]");

    let result = process_commands(&mut byte_cells, snippet);
    assert!(result.is_ok());

    println!("{:#?}", &byte_cells[0..10]);

    assert!(byte_cells[0] == 0);
    assert!(byte_cells[1] == 8);
}

#[test]
fn test_cell_underflow() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from(">>>-");

    let result = process_commands(&mut byte_cells, snippet);

    assert!(result.is_err());
    let result = result.unwrap_err();

    let position = match result {
        Error::CellUnderflow { position } => position,
        _ => panic!(),
    };

    assert!(position == 3);
}

#[test]
fn test_cell_overflow() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from(
        r###"
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
        ++++++++++++++++++++++++++++++++++++++++
    "###,
    );

    let result = process_commands(&mut byte_cells, snippet);

    assert!(result.is_err());
    let result = result.unwrap_err();

    let position = match result {
        Error::CellOverflow { position } => position,
        _ => panic!(),
    };

    assert!(position == 0);
}

#[test]
fn test_pointer_out_of_left_bound() {
    let mut byte_cells = create_byte_cells();

    let snippet = String::from("<");

    let result = process_commands(&mut byte_cells, snippet);

    assert!(result.is_err());
    let result = result.unwrap_err();

    match result {
        Error::PointerOutOfLeftBound => {}
        _ => panic!(),
    };
}

#[test]
fn test_pointer_out_of_right_bound() {
    let mut byte_cells = create_byte_cells();

    let mut snippet = String::new();

    for _ in 0..35_000 {
        snippet.push('>')
    }

    let result = process_commands(&mut byte_cells, snippet);

    assert!(result.is_err());
    let result = result.unwrap_err();

    match result {
        Error::PointerOutOfRightBound => {}
        _ => panic!(),
    };
}
