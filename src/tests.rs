use crate::*;

#[test]
fn test_creation() {
    let b = Brainfuck::new();
    assert_eq!(b.cells, [0; 30000]);
    assert_eq!(b.code_pointer, 0);
    assert_eq!(b.cell_pointer, 0);
}

#[test]
fn test_add() {
    let mut b = Brainfuck::new();
    let code = String::from("++++");
    b.run_stream(code);
    assert_eq!(b.cells[0], 4);
    assert_ne!(b.cells[0], 3);
    assert_ne!(b.cells[0], 5);
    assert_eq!(b.cells[1], 0);
}

#[test]
fn test_sub() {
    let mut b = Brainfuck::new();
    let code = String::from("---");
    b.run_stream(code);
    assert_eq!(b.cells[0], 253);
    assert_ne!(b.cells[0], 0);
    assert_eq!(b.cells[1], 0);
}

#[test]
fn test_move_right() {
    let mut b = Brainfuck::new();
    let code = String::from(">>>>");
    b.run_stream(code);
    assert_eq!(b.cell_pointer, 4);
    assert_ne!(b.cell_pointer, 3);
    assert_ne!(b.cell_pointer, 5);
}

// #[test]
// fn test_add() {
//     let mut b = Brainfuck::new();
//     let code = String::from("++++");
//     b.run_stream(code);
//     assert_eq!(b.cells[0], 4);
//     assert_ne!(b.cells[0], 3);
//     assert_ne!(b.cells[0], 5);
//     assert_eq!(b.cells[1], 0);
// }
