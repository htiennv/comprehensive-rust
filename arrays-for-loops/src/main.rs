#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32;3]; 3]) -> [[i32;3]; 3] {
    let mut transposed = [[0i32; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }

    transposed
    
}

fn pretty_print(matrix: &[[i32;3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }    
}



fn main() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("matrix:");

    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("Matrix transposed:");
    pretty_print(&transposed);
}