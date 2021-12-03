use std::fmt;
// 主函数
fn main() {
    // 将文本打印到命令行
    // println！是一个宏（macros），可以将文本输出到控制台（console）
    println!("Hello, world!");
    // println!("I'm a favers");

    // 各种注释
    // comments();
    // 格式化输出
    // format_log();
    // debug 输出 {:?} 或者 {:#?}
    // debug_print();
    // display 输出 {}
    // display_print();
    // display_min_max();
    // display_complex();
    display_list();
}

fn comments() {
    //--- // 单行注释，注释内容直到行尾
    //--- /* 块注释， 注释内容一直到结束分隔符 */
    //--- /// 为接下来的项生成帮助文档
    //--- //! 为注释所属于的项（译注：如 crate、模块或函数）生成帮助文档
}

fn format_log() {
    /*
    format!：将格式化文本写到字符串（String）。
    print!：与 format！类似，但将文本输出到控制台（io::stdout）。
    println!: 与 print！类似，但输出结果追加一个换行符。
    eprint!：与 format！类似，但将文本输出到标准错误（io::stderr）。
    eprintln!：与 eprint！类似，但输出结果追加一个换行符。
    */

    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);
    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。
    println!("{} days", 31i64);

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // println！会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。

    let pi = 3.141592;
    println!("Pi is roughly {1:.0$}", 3, pi);
}

fn debug_print() {
    // 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
    struct UnPrintable(i32);

    // `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // println!("test {}", UnPrintable(1));
    println!("DebugPrintable {:?}", DebugPrintable(1));

    // 推导 `Structure` 的 `fmt::Debug` 实现。
    // `Structure` 是一个包含单个 `i32` 的结构体。
    #[derive(Debug)]
    struct Structure(i32);

    // 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
    #[derive(Debug)]
    struct Deep(Structure);

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}

fn display_print() {
    // 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
    // `Structure`，包含一个 `i32` 元素。
    struct Structure(i32);

    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
    impl fmt::Display for Structure {
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
            // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
            write!(f, "{}", self.0)
        }
    }

    println!("Structure {}", Structure(123));
}

fn display_min_max() {
    // 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
    #[derive(Debug)]
    struct MinMax(i64, i64);
    println!("debug {:?}", MinMax(1, 2));

    // 实现 `MinMax` 的 `Display`。
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 使用 `self.number` 来表示各个数据。
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    println!("display {}", MinMax(2, 4));

    // 为了比较，定义一个含有具名字段的结构体。
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 自定义格式，使得仅显示 `x` 和 `y` 的值。
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 1.0, y: 2.0 };
    println!("{:?}", point);
    println!("{}", point);
    // println!("{:b}", point);
}

fn display_complex() {
    #[derive(Debug)]
    struct Complex {
        real: f32,
        imag: f32,
    }
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("{:?}", complex);

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}!", self.real, self.imag)
        }
    }
    println!("{}", complex);
}

fn display_list() {
    // 定义一个包含单个 `Vec` 的结构体 `List`。
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // 使用元组的下标获取值，并创建一个 `vec` 的引用。
            let vec = &self.0;
            write!(f, "[")?;
            // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
            for (count, v) in vec.iter().enumerate() {
                // 对每个元素（第一个元素除外）加上逗号。
                // 使用 `?` 或 `try!` 来返回错误。
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", count, v)?;
            }

            // 加上配对中括号，并返回一个 fmt::Result 值。
            write!(f, "]")
        }
    }
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
