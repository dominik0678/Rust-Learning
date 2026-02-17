fn add_task(tasks: &mut Vec<String>, name: &str) {
    // TODO 1:
    // add a new task at the end of the vector
    tasks.push(name.to_string());
}
fn mark_second(tasks: &mut Vec<String>) {
    // TODO 2:
    // on the second task (index 1):
    // 1) append " [done]" with push_str
    // 2) append '!' with push
    tasks[1].push_str(" [done]");
    tasks[1].push('!');
}
fn pop_last_task(mut tasks: Vec<String>) -> (Vec<String>, String) {
    // TODO 3:
    // take ownership, pop last task, return (remaining, popped)
    // tip: pop().expect("...")
    let last = tasks.pop().expect("tasks should not be empty");
    (tasks, last)
}
fn main() {
    let mut tasks = vec![
        String::from("plan"),
        String::from("build"),
    ];
    add_task(&mut tasks, "test");
    println!("count: {}", tasks.len());
    mark_second(&mut tasks);
    println!("second: {}", tasks[1]);
    let (tasks, last) = pop_last_task(tasks);
    println!("popped: {last}");
    println!("remaining: {}", tasks.len());
}