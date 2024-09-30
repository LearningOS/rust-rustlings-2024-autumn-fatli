use std::cmp::Ord;

pub struct Heap<T>
where
    T: Default + Ord,
{
    items: Vec<T>, // 存储堆的元素
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    // 创建一个新的空堆
    pub fn new() -> Self {
        Heap { items: Vec::new() }
    }

    // 获取堆的长度
    pub fn len(&self) -> usize {
        self.items.len()
    }

    // 检查堆是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // 添加元素到堆中
    pub fn add(&mut self, value: T) {
        self.items.push(value); // 将新元素添加到底部
        self.bubble_up(self.len() - 1); // 上浮元素
    }

    // 上浮操作
    fn bubble_up(&mut self, idx: usize) {
        let mut index = idx;
        while index > 0 {
            let parent_index = self.parent_idx(index);
            if self.items[index] < self.items[parent_index] {
                // 最小堆
                self.items.swap(index, parent_index);
                index = parent_index; // 更新到父节点
            } else {
                break; // 如果父节点更小，停止上浮
            }
        }
    }

    // 提取堆顶元素
    pub fn extract(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 如果堆为空，返回 None
        }

        let last_index = self.len() - 1;
        self.items.swap(0, last_index); // 交换堆顶和最后一个元素
        let min_value = self.items.pop(); // 弹出最大（或最小）元素
        self.bubble_down(0); // 重新调整堆
        min_value // 返回提取的值
    }

    // 下沉操作
    fn bubble_down(&mut self, idx: usize) {
        let len = self.len();
        let mut index = idx;

        loop {
            let left_child = self.left_child_idx(index);
            let right_child = self.right_child_idx(index);
            let mut smallest = index;

            if left_child < len && self.items[left_child] < self.items[smallest] {
                smallest = left_child; // 更新为左子节点
            }
            if right_child < len && self.items[right_child] < self.items[smallest] {
                smallest = right_child; // 更新为右子节点
            }
            if smallest != index {
                self.items.swap(index, smallest); // 交换位置
                index = smallest; // 更新索引到最小的子节点
            } else {
                break; // 如果没有交换，停止下沉
            }
        }
    }

    // 获取父节点的索引
    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    // 获取左子节点的索引
    fn left_child_idx(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    // 获取右子节点的索引
    fn right_child_idx(&self, idx: usize) -> usize {
        2 * idx + 2
    }
}

fn main() {
    let mut heap = Heap::new();

    heap.add(5);
    heap.add(3);
    heap.add(8);
    heap.add(1);

    println!("Extracted items from heap:");
    while let Some(min) = heap.extract() {
        println!("{}", min); // 打印提取的最小值
    }
}
