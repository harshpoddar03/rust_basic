

    pub fn transpose(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len(){
            for j in i..matrix[0].len(){
                let mut temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }

    pub fn reverseRows(matrix: &mut Vec<Vec<i32>>){
        for i in 0..matrix.len() {
            let mut left = 0;
            let mut right = matrix.len() - 1;

            while(left < right) {
                let mut temp = matrix[i][left];
                matrix[i][left] = matrix[i][right];
                matrix[i][right] = temp;

                left += 1;
                right -= 1;
            }
        }
    }


fn main() {
    let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    transpose(&mut matrix);
    reverseRows(&mut matrix);
    println!("{:?}", matrix);
}