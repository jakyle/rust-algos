pub fn add_binary(a: String, b: String) -> String {
    let mut a = u128::from_str_radix(&a, 2).unwrap();
    let mut b = u128::from_str_radix(&b, 2).unwrap();
    let (mut carry, mut answer) = (0, 0);
    while b != 0 {
        answer = a ^ b;
        carry = (a & b) << 1;
        a = answer;
        b = carry;
    }
    format!("{:b}", a)
}