// into_iter 会夺走所有权
// iter 是借用
// iter_mut 是可变借用
fn do_something() {
    let values = vec![1, 2, 3];
    // zip，map，filter 是迭代器适配器：
    //
    // zip 把两个迭代器合并成一个迭代器，新迭代器中，每个元素都是一个元组，由之前两个迭代器的元素组成。
    // 例如将形如 [1, 2, 3, 4, 5] 和 [2, 3, 4, 5] 的迭代器合并后，新的迭代器形如 [(1, 2),(2, 3),(3, 4),(4, 5)]
    // map 是将迭代器中的值经过映射后，转换成新的值[2, 6, 12, 20]
    // filter 对迭代器中的元素进行过滤，若闭包返回 true 则保留元素[6, 12]，反之剔除
    // 而 sum 是消费者适配器，对迭代器中的所有元素求和，最终返回一个 u32 值 18。

    let values2: Vec<_> = values.iter().map(|x| { x + 1 }).collect();

    println!("{values2:?},{values:?}")
}

#[cfg(test)]
mod iter_test {
    use super::*;

    #[test]
    fn test01() {
        do_something()
    }
}