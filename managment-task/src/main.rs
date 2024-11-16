#[derive(Debug, Clone, Copy)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    description: String,
    status: TaskStatus,
}

struct TaskManager {
    list_task: Vec<Task>,
}

impl TaskManager {
    fn create(create_list_task: Vec<Task>) -> TaskManager {
        TaskManager {
            list_task: create_list_task,
        }
    }

    fn add_new_task(&mut self, new_task: Task) {
        self.list_task.push(new_task);
    }
    fn update_status(&mut self, id: i32, new_status: &TaskStatus) {
        self.list_task.iter_mut().for_each(|task| {
            if task.id == id {
                task.status = *new_status;
            }
        })
    }
    fn print(&self) {
        self.list_task.iter().for_each(|x| {
            println!(
                "Task ID: {}, Name: {}, Description: {}, Status: {:?}",
                x.id, x.name, x.description, x.status
            )
        });
    }
    fn delete(&mut self, id: i32) {
        self.list_task.retain(|task| task.id != id);
    }
}

fn main() {
    let task1 = Task {
        id: 1,
        name: String::from("Task 1"),
        description: String::from("Description 1"),
        status: TaskStatus::Pending,
    };
    let task2 = Task {
        id: 2,
        name: String::from("Task 2"),
        description: String::from("Description 2"),
        status: TaskStatus::InProgress,
    };
    let task3 = Task {
        id: 3,
        name: String::from("Task 3"),
        description: String::from("Description 3"),
        status: TaskStatus::Completed,
    };
    let task4 = Task {
        id: 4,
        name: String::from("Task 4"),
        description: String::from("Description 4"),
        status: TaskStatus::Pending,
    };
    let task5 = Task {
        id: 5,
        name: String::from("Task 5"),
        description: String::from("Description 5"),
        status: TaskStatus::InProgress,
    };

    let mut task_manager = TaskManager::create(vec![task1, task2, task3, task4]);

    // add new task
    println!("show witout changes");
    task_manager.print();

    task_manager.add_new_task(task5);

    println!("show add task");
    task_manager.print();

    task_manager.update_status(5, &TaskStatus::Completed);

    println!("update the task 5");
    task_manager.print();

    task_manager.delete(5);
    println!("delete the task 5");
    task_manager.print();
}
