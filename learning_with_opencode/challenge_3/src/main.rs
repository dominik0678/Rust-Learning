fn total_chars(tasks: &[String]) -> usize {
    // TODO 1:
    // return total number of characters of all task strings
    // use: let mut total = 0; + for loop
    let mut total = 0;
    for t in tasks {
        total += t.len();
    }
    total
}
fn mark_second_done(tasks: &mut Vec<String>) {
    // TODO 2:
    // append " [done]" only to the second task (index 1)
    tasks[1].push_str(" [done]")
}
fn pop_last_task(mut tasks: Vec<String>) -> (Vec<String>, String) {
    // TODO 3:
    // take ownership of tasks
    // remove last task
    // return (remaining_tasks, removed_task)
    // tip: pop().expect("...")
    let last = tasks.pop().expect("tasks should not be empty");
    (tasks, last)
}
fn main() {
    let mut tasks = vec![
        String::from("plan"),
        String::from("code"),
        String::from("test"),
    ];
    let total = total_chars(&tasks);
    println!("total chars: {total}");
    mark_second_done(&mut tasks);
    println!("second: {}", tasks[1]);
    let (tasks, last) = pop_last_task(tasks);
    println!("popped: {last}");
    println!("remaining: {}", tasks.len());
}