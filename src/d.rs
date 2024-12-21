use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
struct BigNumber {
    value: String,
}

impl BigNumber {
    // 创建一个新的BigNumber实例
    fn new(num_str: &str) -> BigNumber {
        BigNumber {
            value: num_str.to_string(),
        }
    }

    // 加法运算
    fn add(&self, other: &BigNumber) -> BigNumber {
        let mut result = String::new();
        let mut carry = 0;
        let mut i = self.value.len() - 1;
        let mut j = other.value.len() - 1;

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;
            if i >= 0 {
                sum += self.value.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
                i -= 1;
            }
            if j >= 0 {
                sum += other.value.chars().nth(j).unwrap().to_digit(10).unwrap() as usize;
                j -= 1;
            }
            result.push((sum % 10).to_string().chars().next().unwrap());
            carry = sum / 10;
        }

        result.chars().rev().collect::<String>().into()
    }

    // 减法运算
    fn sub(&self, other: &BigNumber) -> BigNumber {
        if self < other {
            panic!("减法结果为负数，暂不支持");
        }

        let mut result = String::new();
        let mut borrow = 0;
        let mut i = self.value.len() - 1;
        let mut j = other.value.len() - 1;

        while i >= 0 || j >= 0 {
            let mut diff = borrow;
            if i >= 0 {
                diff += self.value.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
                i -= 1;
            }
            if j >= 0 {
                diff -= other.value.chars().nth(j).unwrap().to_digit(10).unwrap() as usize;
                j -= 1;
            }

            if diff < 0 {
                diff += 10;
                borrow = -1;
            } else {
                borrow = 0;
            }

            result.push((diff % 10).to_string().chars().next().unwrap());
        }

        result.chars().rev().collect::<String>().into()
    }

    // 乘法运算
    fn mul(&self, other: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new("0");
        for (i, digit_other) in other.value.chars().enumerate() {
            let mut carry = 0;
            let mut temp_result = String::new();
            for digit_self in self.value.chars().rev() {
                let product = digit_self.to_digit(10).unwrap() as usize * digit_other.to_digit(10).unwrap() as usize + carry;
                temp_result.push((product % 10).to_string().chars().next().unwrap());
                carry = product / 10;
            }
            if carry > 0 {
                temp_result.push((carry % 10).to_string().chars().next().unwrap());
            }
            result = result.add(&BigNumber::new(&temp_result.chars().rev().collect::<String>()));
            result = result.add(&BigNumber::new("0".repeat(i)));
        }
        result
    }

    // 除法运算
    fn div(&self, other: &BigNumber) -> BigNumber {
        if other.value == "0" {
            panic!("除数不能为零");
        }

        let mut dividend = self.clone();
        let mut quotient = BigNumber::new("0");

        while dividend >= *other {
            dividend = dividend.sub(other);
            quotient = quotient.add(&BigNumber::new("1"));
        }

        quotient
    }
}

// 实现比较大小的方法，用于减法等运算中的判断
impl PartialEq for BigNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BigNumber {}

impl PartialOrd for BigNumber {
    fn partial_cord(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for BigNumber {
    fn cord(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

// 实现格式化输出的方法，方便打印
impl fmt::Display for BigNumber {
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let num1 = BigNumber::new("12345678901234567890");
    let num2 = BigNumber::new("98765432109876543210");

    let sum = num1.add(num2);
    let diff = num1.sub(num2);
    let product = num1.mul(num2);
    let quotient = num1.div(num2);

    println!("{} + {} = {}", num1, num2, sum);
    println!("{} - {} = {}", num1, num2, diff);
    println!("{} * {} = {}", num1, num2, product);
    println!("{} / {} = {}", num1, num2, quotient);
}
