Rust中`generators`的特性允许我们定义生成器或协程字面。生成器是一个“可恢复函数”，在语法上类似于闭包，但在编译器本身中可编译为多种不同的语义。生成器的主要特征是可以在执行过程中将其挂起，以在以后恢复。生成器使用`yield`关键字返回，然后调用者可以`resume`使生成器在`yield`关键字之后立即恢复执行。

生成器的语法示例是：

```rust
#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        yield 1;
        return "foo"
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected value from resume"),
    }
}
```

生成器是类似闭包的写法，可以包含一条`yield`语句。该 `yield`语句采用可选表达式来让出`Generator`。该`Generator trait`有一个主要方法，`resume`方法在先前的暂停点恢复继续执行。

目前，生成器的主要预期用例是异步/等待语法的实现原语，但是将来，生成器可能会扩展到迭代器和其他原语的人体工程学实现。

####  `Generator Trait`

当前的`Generator Trait`定义如下：

```rust
pub trait Generator<R = ()> {
    type Yield;
    type Return;
    fn resume(self: Pin<&mut Self>, resume: R) -> GeneratorState<Self::Yield, Self::Return>;
}
```

`Generator::Yield`类型是被 `yield`语句产生的类型。`Generator::Return`类型是Generator的返回类型。这通常是生成器定义中的最后一个表达式，或者`return`是生成器中传递给它的任何值。该`resume`方法是执行`Generator`的入口。

`resume`的返回值`GeneratorState`是这样的：

```rust
pub enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}
```

`Yielded`变量表示生成器可以稍后恢复。这对应Generator中的一个`yield`点。`Complete`变量表示生成器已完成，无法再次恢复。生成器返回`Complete`后调用`resume` 会panic。

#### 类闭包的语义

生成器类似闭包的语法暗示着它们也具有类似闭包的语义。

- 创建后，生成器不执行任何代码。闭包实际上不会在构造时执行任何闭包代码，并且类似地，生成器在构造时不会在生成器内部执行任何代码。
- 生成器可以通过引用或移动来捕获外部变量，并且可以`move`在闭包开始时使用关键字对其进行调整。像闭包一样，所有生成器都将具有由编译器推断的隐式环境。可以将外部变量移至生成器中，以随着生成器的进行使用。
- 生成器产生具有唯一类型的值，该值实现 `std::ops::Generator trait`。这允许通过该`Generator::resume`方法实际执行生成器，也可以将其命名为返回类型等。
- 根据捕获的环境变量，自动为实施`Send`和`Sync`这样的特性`Generator`。与闭包不同，生成器还依赖于悬挂点之间的变量。这意味着，尽管周围环境可能是`Send`或`Sync`，但发电机本身可能不是由于跨`yield`非`Send`或非跨点的内部变量所致`Sync`。请注意，生成器不会实现类似`Copy`或`Clone`自动的特征。
- 每当生成器被丢弃时，它将丢弃所有捕获的环境变量。

### Generators 作为状态机

在编译器中，生成器当前被编译为状态机。每个 `yield`表达式将对应于一个不同的状态，所有状态存储该暂停点上的活动变量。恢复生成器将分派当前状态，然后直到内部执行达到`yield` ，此时所有状态都保存在生成器中并返回值。

```rust
#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

fn main() {
    let ret = "foo";
    let mut generator = move || {
        yield 1;
        return ret
    };

    Pin::new(&mut generator).resume(());
    Pin::new(&mut generator).resume(());
}
```

上面代码将会被编译为以下内容：

```rust
fn main() {
    let ret = "foo";
    let mut generator = {
        enum __Generator {
            Start(&'static str),
            Yield1(&'static str),
            Done,
        }

        impl Generator for __Generator {
            type Yield = i32;
            type Return = &'static str;

            fn resume(mut self: Pin<&mut Self>, resume: ()) -> GeneratorState<i32, &'static str> {
                use std::mem;
                match mem::replace(&mut *self, __Generator::Done) {
                    __Generator::Start(s) => {
                        *self = __Generator::Yield1(s);
                        GeneratorState::Yielded(1)
                    }

                    __Generator::Yield1(s) => {
                        *self = __Generator::Done;
                        GeneratorState::Complete(s)
                    }

                    __Generator::Done => {
                        panic!("generator resumed after completion")
                    }
                }
            }
        }
        __Generator::Start(ret)
    };

    Pin::new(&mut generator).resume(());
    Pin::new(&mut generator).resume(());
}
```

当生成器启动时，它会立即 yield 1，但是它会在退出之前保存其状态，表明它已经达到yield点。再次恢复后，我们将执行`return ret`返回`Complete` 状态的。在这里，我们还可以注意到`Done`，如果恢复该状态，则立即会发生panic，因为恢复已完成的生成器是无效的。还值得注意的是，这仅仅是粗略的解释，而不是针对编译器功能的规范说明。
