fn total_len(tasks: &[String]) -> usize {
    // TODO: return total number of characters across all tasks
    tasks.iter().map(|t| t.len()).sum()
}
fn mark_done(task: &mut String) {
    // TODO: append " [done]" to the task
    task.push_str(" [done]");
}
fn consume_last(mut tasks: Vec<String>) -> (Vec<String>, String) {
    // TODO:
    // - take ownership of the vector
    // - remove the last task
    // - return (remaining_tasks, removed_task)
    // tip: use pop().expect("...")
    let last = tasks.pop().expect("tasks should not be empty");
    (tasks, last)
}
fn main() {
    let mut tasks = vec![
        String::from("learn"),
        String::from("borrow"),
        String::from("ffi"),
    ];
    let total = total_len(&tasks);
    println!("total chars: {total}");
    mark_done(&mut tasks[1]);
    println!("second task: {}", tasks[1]);
    let (tasks, last) = consume_last(tasks);
    println!("popped: {last}");
    println!("remaining: {}", tasks.len());
}