#### 箱（Crate）

箱"是二进制程序文件或者库文件，存在于"包"中。"箱"是树状结构的，它的树根是编译器开始运行时编译的源文件所编译的程序。

注意："二进制程序文件"不一定是"二进制可执行文件"，只能确定是是包含目标机器语言的文件，文件格式随编译环境的不同而不同。

#### 包（Package）

工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库箱还是二进制箱）。

当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。

#### 模块（Module）

*模块*让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性。模块还可以控制项的 *私有性*，即项是可以被外部代码使用的（*public*），还是作为一个内部实现的内容，不能被外部代码使用（*private*）。

定义一个模块，是以 `mod` 关键字为起始，然后指定模块的名字，并且用花括号包围模块的主体。在模块内，我们还可以定义其他的模块。模块还可以保存一些定义的其他项，比如结构体、枚举、常量、特性、或者函数。

```rust
pub mod my_mod {
    pub struct person {
        pub name:string
    }
}
```

#### 访问权限

如果模块中定义了结构体，结构体除了其本身是私有的以外，其字段也默认是私有的。所以如果想使用模块中的结构体以及其字段，需要 pub 声明，枚举类枚举项可以内含字段，但不具备类似的性质。

```rust
mod my_mod {
    pub struct person {
        pub name: String
    }

    impl person {
        pub fn new(name_str: String) -> person {
            person {
                name: name_str
            }
        }
    }

    pub enum sex {
        m,
        f,
    }
}

fn main() {
    let p = my_mod::person::new(String::from("four"));
    println!("{}", p.name);
    let sex = my_mod::sex::m;
    
}
```


#### use

use 关键字能够将模块标识符引入当前作用域，在作用域中增加 use 和路径类似于在文件系统中创建软连接。
```rust
mod my_mod {
    pub struct person {
        pub name: String
    }

    impl person {
        pub fn new(name_str: String) -> person {
            person {
                name: name_str
            }
        }
    }

    pub enum sex {
        m,
        f,
    }
}

//use crate::mods::my_mod::person;
//use crate::mods::my_mod::sex;
//use crate::mods::my_mod::*;
use my_mod::*;
//use my_mod as m;
fn main() {
    let p = person::new(String::from("four"));
    println!("{}", p.name);
    let sex = sex::m;
}
```

当使用 `use` 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 `pub` 和 `use`。这个技术被称为 “*重导出*（*re-exporting*）”，因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。

#### mod 

当模块变得更大时，可能想要将它们的定义移动到单独的文件中，从而使代码更容易阅读。

```rust
// common.rs
pub mod utils{
    pub fn print(str : & String) {
        println!("{}", str)
    }
}
```

```rust
// main.rs
mod common
fn main() {
    common::utils::print(&String::from("aa"));
}
```

在 `mod common`后使用分号，这将告诉 Rust 在另一个common文件中加载模块的内容。