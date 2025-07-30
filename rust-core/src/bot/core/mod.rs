use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Status of a [`Task`] within the queue.
#[derive(Clone, Debug, serde::Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// A unit of work for the `KAIROBOT`.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub command: String,
    pub status: TaskStatus,
}

/// A simple in-memory queue for [`Task`]s.
#[derive(Default)]
pub struct TaskQueue {
    pub tasks: Vec<Task>,
}

impl TaskQueue {
    /// Create a new empty queue.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Add a task to the queue and return its generated ID.
    pub fn add_task(&mut self, task: Task) -> String {
        let id = task.id.clone();
        self.tasks.push(task);
        id
    }

    /// Add a task to the queue and return its generated ID.
    pub fn push(&mut self, name: impl Into<String>, command: impl Into<String>) -> String {
        let id = Uuid::new_v4().to_string();
        self.tasks.push(Task {
            id: id.clone(),
            name: name.into(),
            command: command.into(),
            status: TaskStatus::Pending,
        });
        id
    }

    /// Gets the next pending task and sets its status to [`TaskStatus::InProgress`].
    pub fn get_next_task(&mut self) -> Option<Task> {
        if let Some(task) = self
            .tasks
            .iter_mut()
            .find(|t| matches!(t.status, TaskStatus::Pending))
        {
            task.status = TaskStatus::InProgress;
            return Some(task.clone());
        }
        None
    }

    /// Updates the status of a task by its ID.
    pub fn update_task_status(&mut self, task_id: &str, status: TaskStatus) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            println!("Core: Updating status for task '{}' to {:?}", task_id, status);
            task.status = status;
        }
    }
}

/// The main loop of the KAIROBOT.
pub async fn main_loop(queue: Arc<Mutex<TaskQueue>>) {
    println!("KAIROBOT Core: Main loop started. Monitoring task queue...");
    loop {
        let task_to_run;
        {
            let mut q = queue.lock().await;
            task_to_run = q.get_next_task();
        }

        if let Some(task) = task_to_run {
            println!("Core: Executing task '{}' ({})", task.name, task.id);
            // TODO: Dispatch to the plugin layer
            let success = crate::bot::plugin::shell::execute(&task.command).await;

            let final_status = if success {
                TaskStatus::Completed
            } else {
                TaskStatus::Failed
            };
            {
                let mut q = queue.lock().await;
                q.update_task_status(&task.id, final_status);
            }
        } else {
            sleep(Duration::from_secs(2)).await;
        }
    }
}
