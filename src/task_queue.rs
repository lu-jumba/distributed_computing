use std::collections::VecDeque;

struct TaskQueue {
    queue: VecDeque<Task>,
}

impl TaskQueue {
    // Add a task to the queue
    fn enqueue(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    // Dequeue the next task from the queue
    fn dequeue(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }
}
