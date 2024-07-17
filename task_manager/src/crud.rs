use diesel::prelude::*;
use crate::models::{Task, NewTask};
use crate::schema::tasks;

pub fn create_task(conn: &mut SqliteConnection, title: &str, description: Option<&str>) -> QueryResult<Task> {
    let new_task = NewTask {
        title,
        description,
        completed: false,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)?;

    tasks::table.order(tasks::id.desc()).first(conn)
}

pub fn read_tasks(conn: &mut SqliteConnection) -> QueryResult<Vec<Task>> {
    tasks::table.load::<Task>(conn)
}

pub fn update_task(conn: &mut SqliteConnection, task_id: i32, completed: bool) -> QueryResult<Task> {
    diesel::update(tasks::table.find(task_id))
        .set(tasks::completed.eq(completed))
        .execute(conn)?;

    tasks::table.find(task_id).get_result(conn)
}

pub fn delete_task(conn: &mut SqliteConnection, task_id: i32) -> QueryResult<usize> {
    diesel::delete(tasks::table.find(task_id))
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;

    #[test]
    fn test_crud_operations() {
        let mut conn = establish_connection();

        // Create
        let task = create_task(&mut conn, "Test Task", Some("Test Description")).unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test Description".to_string()));
        assert_eq!(task.completed, false);

        // Read
        let tasks = read_tasks(&mut conn).unwrap();
        assert!(tasks.len() > 0);

        // Update
        let updated_task = update_task(&mut conn, task.id, true).unwrap();
        assert_eq!(updated_task.completed, true);

        // Delete
        let deleted_count = delete_task(&mut conn, task.id).unwrap();
        assert_eq!(deleted_count, 1);
    }
}