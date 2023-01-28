use std::{time::{Duration, Instant}, collections::HashMap, sync::Arc};
use crate::boundary::main_boundary::{MainBoundary};
use std::collections::BTreeSet;

use super::{server_state::{ServerState}, resources::{TSResources, self}};
pub struct TaskManager {
    tasks: HashMap<Id, Task>,
    id_manager: IdManager,
}

pub enum Schedule {
    Always,
    Tick,
    Wait {
        duration: Duration,
        curr: Instant, 
    },
}

pub struct Task {
    func: Box<dyn FnMut(&TSResources)>,
    schedule: Schedule,
}

type Id = usize;

pub struct IdManager {
    ids: BTreeSet<usize>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            id_manager: IdManager::new(),
        }
    }

    pub fn register_task(&mut self, func: impl FnMut(&TSResources), schedule: Schedule) -> Id {
        let id = self.id_manager.generate_id();
        
        self.tasks.insert(id, Task::new(Box::new(func), schedule));

        id
    }

    pub fn unregister_task(&mut self, id: Id) {
        self.tasks.remove(&id);
    }


    pub fn poll_and_execute_all(&mut self, resources: &TSResources, is_tick: bool) {
        for (_id, task) in &mut self.tasks {
            match task.schedule {
                Schedule::Always => task.execute(resources),
                Schedule::Tick => if is_tick {
                    task.execute(resources)
                },
                Schedule::Wait { duration, curr } => {
                    if curr.elapsed() > duration {
                        task.execute(resources);

                        task.schedule = Schedule::Wait { duration, curr: Instant::now() };
                    }
                },
            }
        }
    }
}

impl Task {
    pub fn new(func: Box<dyn FnMut(&TSResources)>, schedule: Schedule) -> Self {
        Self {
            func,
            schedule,
        }
    }

    pub fn execute(&mut self, resources: &TSResources) {
        (self.func)(resources);
    }
}

impl IdManager {
    fn new() -> Self {
        Self {
            ids: BTreeSet::new(),
        }
    }

    fn generate_id(&mut self) -> Id {
        for id in 0usize.. {
            if self.ids.insert(id) {
                return id;
            }
        }

        return 0;
    }
}
