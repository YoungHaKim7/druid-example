use std::{fs, path::Path};

use directories::BaseDirs;
use druid::Widget;
use serde::{Deserialize, Serialize};

use crate::data::{TodoItem, TodoState};

pub struct Saver;

impl Widget<TodoState> for Saver {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut TodoState,
        env: &druid::Env,
    ) {
    }
    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoState,
        env: &druid::Env,
    ) {
    }
    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &druid::Env,
    ) {
        if data.todos != old_data.todos {
            if let Some(base_dirs) = BaseDirs::new() {
                let config = format!(
                    "{}/{}",
                    base_dirs.config_dir().to_str().unwrap(),
                    "MyTodo.json"
                );
                let config_path = Path::new(&config);
                let tasks = TaskData {
                    tasks: data.todos.clone().into_iter().collect(),
                };
                fs::write(config_path, serde_json::to_string(&tasks).unwrap())
                    .expect("Config path does not fully exist");
            }
        }
    }
    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &TodoState,
        env: &druid::Env,
    ) -> druid::Size {
        druid::Size {
            width: 0.,
            height: 0.,
        }
    }
    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &TodoState, env: &druid::Env) {}
}

#[derive(Serialize, Deserialize)]
pub struct TaskData {
    pub tasks: Vec<TodoItem>,
}

pub fn read_stored() -> TaskData {
    if let Some(base_dirs) = BaseDirs::new() {
        let config = format!(
            "{}/{}",
            base_dirs.config_dir().to_str().unwrap(),
            "MyTodo.json"
        );
        let config_path = Path::new(&config);
        let data = match fs::read_to_string(config_path) {
            Ok(a) => a,
            Err(_) => return TaskData { tasks: Vec::new() },
        };
        match serde_json::from_str(&data) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("The save data is corrupted or no longer in the format it shoutld be in Error{}", e);
                return TaskData { tasks: Vec::new() };
            }
        }
    } else {
        return TaskData { tasks: Vec::new() };
    }
}
