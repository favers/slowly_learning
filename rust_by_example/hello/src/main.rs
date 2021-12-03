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
    debug_print();
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
