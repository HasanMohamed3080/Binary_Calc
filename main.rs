use std::io::{self, Write};

fn main() {
    // إنشاء متغير لاحتواء الرقم الذي يقوم المستخدم بإدخاله
    let mut input = String::new();

    // استقبال الرقم أو النص من المستخدم
    print!("Enter a decimal number or text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // تحويل الرقم أو النص إلى نظام الأعداد الثنائية
    let binary_num = match input.trim().parse::<u64>() {
        Ok(num) => format!("{:b}", num),
        Err(_) => {
            let mut binary_num = String::new();
            for byte in input.trim().as_bytes() {
                binary_num.push_str(&format!("{:08b}", byte));
            }
            binary_num
        }
    };

    // عرض الناتج على الشاشة
    if binary_num.is_empty() {
        println!("Invalid input");
    } else {
        println!("Binary representation: {}", binary_num);
    }
}