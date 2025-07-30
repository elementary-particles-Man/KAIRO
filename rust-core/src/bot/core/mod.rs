use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

const QUEUE_FILE: &str = "task_queue.json";

/// Status of a [`Task`] within the queue.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// A unit of work for the `KAIROBOT`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub command: String,
    pub status: TaskStatus,
}

/// A simple in-memory queue for [`Task`]s.
#[derive(Default, serde::Deserialize, serde::Serialize)]
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

    /// Load the task queue from a file
    pub fn load() -> Self {
        if let Ok(file) = File::open(QUEUE_FILE) {
            let reader = BufReader::new(file);
            if let Ok(queue) = serde_json::from_reader(reader) {
                println!("Core: Task queue loaded from {}", QUEUE_FILE);
                return queue;
            }
        }
        println!("Core: No existing task queue found. Creating a new one.");
        Self::new()
    }

    /// Save the entire task queue to a file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(QUEUE_FILE)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.tasks)?;
        println!("Core: Task queue saved to {}", QUEUE_FILE);
        Ok(())
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