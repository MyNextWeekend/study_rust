// 策略接口
trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

// 具体策略：冒泡排序
struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
        println!("Sorted using BubbleSort: {:?}", data);
    }
}

// 具体策略：快速排序
struct QuickSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        if data.len() < 2 {
            return;
        }
        let pivot = data[0];
        let less: Vec<i32> = data.iter().filter(|&&x| x < pivot).cloned().collect();
        let greater: Vec<i32> = data.iter().filter(|&&x| x > pivot).cloned().collect();

        let mut sorted = Vec::new();
        sorted.extend(less);
        sorted.push(pivot);
        sorted.extend(greater);

        *data = sorted;
        println!("Sorted using QuickSort: {:?}", data);
    }
}

// 上下文
struct SortContext<'a> {
    strategy: &'a dyn SortStrategy,
}

impl<'a> SortContext<'a> {
    fn new(strategy: &'a dyn SortStrategy) -> Self {
        SortContext { strategy }
    }

    fn sort(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
}


#[cfg(test)]
mod strategy_test {
    use super::*;


    #[test]
    fn test_01() {
        let mut data = vec![5, 2, 9, 1, 5, 6];

        // 使用冒泡排序
        let bubble_sort = BubbleSort;
        let context = SortContext::new(&bubble_sort);
        context.sort(&mut data);

        // 使用快速排序
        let quick_sort = QuickSort;
        let context = SortContext::new(&quick_sort);
        context.sort(&mut data);
    }
}

