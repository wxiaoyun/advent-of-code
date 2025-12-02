pub fn print_mat(mat: &Vec<Vec<char>>) {
    for row in mat {
        for ch in row {
            print!("{ch}");
        }
        println!();
    }
}

pub fn parse_mat(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
