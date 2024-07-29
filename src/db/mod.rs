// #[derive(Debug, Clone)]
// pub enum DBAction {
//     Insert,
//     Update,
//     Delete,
//     Select,
// }

// #[derive(Debug, Clone)]
// pub struct DB {
//     connection: String,
//     // command: Option<Command>,
// }

// impl DB {
//     pub fn init() -> Self {
//         DB {
//             // Example connection string
//             connection: "postgres://postgres:password@localhost:5432/postgres".to_string(),
//             // command: None
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Command {
//     action: DBAction,
//     table: Option<String>,
//     fields: Option<Vec<String>>,
//     where_clause: Option<String>,
//     values: Option<Vec<String>>,
// }

// impl Command {
//     pub fn new(action: DBAction) -> Self {
//         Command {
//             action,
//             table: None,
//             fields: None,
//             where_clause: None,
//             values: None,
//         }
//     }

//     pub fn table(&mut self, table: String) -> &mut Self {
//         self.table = Some(table);
//         self
//     }

//     pub fn fields(&mut self, fields: Vec<String>) -> &mut Self {
//         self.fields = Some(fields);
//         self
//     }

//     pub fn where_clause(&mut self, where_clause: String) -> &mut Self {
//         self.where_clause = Some(where_clause);
//         self
//     }

//     pub fn values(&mut self, values: Vec<String>) -> &mut Self {
//         self.values = Some(values);
//         self
//     }
// }
