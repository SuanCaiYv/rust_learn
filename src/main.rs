extern crate core;

mod base;

use std::fmt::Debug;
use crate::base::*;

#[derive(Debug, Clone)]
struct MyType {
    a: i32,
    b: i32,
}

/// T: 'static；是一个类型约束，表示T类型的实例想存活多久都可以，不依赖别的引用，
/// 举例来说，T是一个结构体，它的字段全部是'static类型或者非引用类型，则T是'static类型。
/// 此外，基本类型都是'static类型，例如：i32，f64，char，bool等；此外，对于引用类型来说，只有本身是'static的引用才是。
/// 这么来看其实可以把拥有'static约束的类型理解成“独立类型”，即存活时间取决于自己意愿的类型。
/// 当然了，此类型的变量不一定要存活整个运行期间，可以在运行时被回收的。'static的修饰只是强调它的存活不受限制，而不是存活到程序结束。
/// 而&'static T则是用来修饰引用的，此类型的变量是一个引用类型，表示这个引用背后的数据可以存活任意久。
/// 此外，这还要求T本身是不可变的，且获得这个引用值的变量绝对不能发生所有权转移。后者限定了这个引用只能从构建处获取，任何通过变量获取的引用都是不可行的。
/// 最起码在现阶段编译器没法判断变量后面是否可能被移走的情况下是不可行的；此外，&'static T类型的引用暗示着背后的数据会存活足够长，或者任意长，或者直到程序结束？
/// 最后，可以注意到符合T: 'static的变量一定包含符合&'static T的变量，反之不行。
fn main() {
    let x1 = MyType { a: 1, b: 2 };
    let x1 = MyType { a: 1, b: 2 };
    // x3是&'static MyType
    let x3 = &MyType { a: 1, b: 2 };
    // OK
    f1(&x1);
    // OK, &'static T 一定是'static的
    f2(x3);
    // OK
    f3(x3);
    // 错误示范
    f2(&x2);
}

fn f1<T: 'static>(v: &T) {
}

fn f2<T: 'static>(v: T) {
}

fn f3<T>(v: &'static T) {
}
