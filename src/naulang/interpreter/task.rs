use naulang::interpreter::frame::Frame;

#[derive(PartialEq)]
pub enum TaskState {
    Continue,
    Halt,
    Yield,
    Suspend,
}

/// Represents a independently running function and its call stack
pub struct Task<'task> {
    /// The current state of this task as TaskState
    state:       TaskState,

    /// Represents the top of the current stack this task is running.
    top_frame:   Option<&'task mut Frame>,

    /// The task that spawned this task
    parent_task: Option<Box<Task<'task>>>,
}

impl<'task> Task<'task> {
    pub fn new() -> Task<'task> {
        Task {
            state: TaskState::Continue,
            top_frame: Option::None,
            parent_task: Option::None,
        }
    }

    pub fn set_parent_task(&mut self, task: Task<'task>) -> () {
        self.parent_task =  Some(Box::new(task));
    }
}

#[cfg(test)]
mod tests {
    use super::{Task, TaskState};
    use naulang::interpreter::frame::{Frame, FrameInfo};
    use std::ops::Deref;

    #[test]
    fn test_new_task_state() {
        assert!(Task::new().state == TaskState::Continue);
    }

    #[test]
    fn test_set_parent_task() {
        let mut root_task = Task::new();

        // Create new Task in short lived scope
        {
            // TODO: Proper initialisation, here we're mutating, and then later
            // moving immutable
            let mut next_task = Task::new();
            (*&mut next_task.state) = TaskState::Halt;

            let next_task = next_task;
            root_task.set_parent_task(next_task);
        }

        let task_is_set = match root_task.parent_task {
            Some(rc_task) => rc_task.deref().state == TaskState::Halt,
            None => false
        };

        assert!(task_is_set);
    }
}
