
pub fn step_sum(arr: &[u32]) -> u32
{
    const STEP_SIZE: usize = 8;

    let mut result = 0;

    for step in 0..(arr.len() / STEP_SIZE) {
        result += arr[step * STEP_SIZE];
        // I'd also want to add an offset `i < STEP_SIZE`, but it's not necessary to cause the issue.
        // Or alternatively, replacing the division with `.dev_ceil()`.
    }

    result
}
fn main(){let s=[0,114,514,191,810,999,0,5,0];println!("{}",step_sum(&s))}
