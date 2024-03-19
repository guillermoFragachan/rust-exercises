pub fn square_of_sum(n: u32) -> u32 {
 let sum=(1..=n).sum::<u32>();
 sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
//    let square = (1..=n).product: :<u32>();

    let mut sum = 0;

   for num in  1..=n { 
      let square =  num * num;
      sum += square
   }

        sum
}

pub fn difference(n: u32) -> u32 {

    let sum_of_squares = sum_of_squares(n);
    let square_of_sum = square_of_sum(n);

    let difference = square_of_sum - sum_of_squares;

    difference

    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
}
