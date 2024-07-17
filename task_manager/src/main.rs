use structopt::StructOpt;
use task_manager::{db, crud};

#[derive(StructOpt)]
enum Command {
    Add {
        title: String,
        #[structopt(short, long)]
        description: Option<String>,
    },
    List,
    Complete {
        id: i32,
    },
    Delete {
        id: i32,
    },
}

fn main() {
    let command = Command::from_args();
    let mut conn = db::establish_connection();

    match command {
        Command::Add { title, description } => {
            match crud::create_task(&mut conn, &title, description.as_deref()) {
                Ok(task) => println!("Added task: ID: {}, Title: {}, Description: {:?}, Completed: {}", 
                                     task.id, task.title, task.description, task.completed),
                Err(e) => eprintln!("Error adding task: {}", e),
            }
        }
        Command::List => {
            match crud::read_tasks(&mut conn) {
                Ok(tasks) => {
                    for task in tasks {
                        println!("ID: {}, Title: {}, Description: {:?}, Completed: {}",
                                 task.id, task.title, task.description, task.completed);
                    }
                }
                Err(e) => eprintln!("Error listing tasks: {}", e),
            }
        }
        Command::Complete { id } => {
            match crud::update_task(&mut conn, id, true) {
                Ok(task) => println!("Completed task: ID: {}, Title: {}", task.id, task.title),
                Err(e) => eprintln!("Error completing task: {}", e),
            }
        }
        Command::Delete { id } => {
            match crud::delete_task(&mut conn, id) {
                Ok(num_deleted) => println!("Deleted {} task(s)", num_deleted),
                Err(e) => eprintln!("Error deleting task: {}", e),
            }
        }
    }
}