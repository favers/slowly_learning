fn main() {
    // Rust 提供了多种原生类型 (primitive)
    /*
    标量类型（scalar type）
    有符号整型（signed integers）：i8、i16、i32、i64 和 isize（指针宽度）
    无符号整型（unsigned integers）： u8、u16、u32、u64 和 usize（指针宽 度）
    浮点类型（floating point）： f32、f64
    char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
    bool（布尔型）：只能是 true 或 false
    单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组

    复合类型（compound type）
    数组（array）：如 [1, 2, 3]
    元组（tuple）：如 (1, true)
    */
    // primitive()
    literals()
}

fn primitive() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0; // 常规说明
    let an_integer = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 报错！变量的类型并不能改变。
    // mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}

fn literals() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要

    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
}
