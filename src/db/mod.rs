enum DBAction {
    Insert,
    Update,
    Delete,
    Select,
}

pub struct DB {
    connection: String,
    command: Option<Command>,
}

struct Command {
    action: DBAction,
    table: Option<String>,
    fields: Option<Vec<String>>,
    where_clause: Option<String>,
    values: Option<Vec<String>>,
}

impl Command {
    pub fn new(action: DBAction) -> Self {
        Command {
            action,
            table: None,
            fields: None,
            where_clause: None,
            values: None,
        }
    }

    fn table(&mut self, table: String) -> &mut Self {
        self.table = Some(table);
        self
    }

    fn fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    fn where_clause(&mut self, where_clause: String) -> &mut Self {
        self.where_clause = Some(where_clause);
        self
    }

    fn values(&mut self, values: Vec<String>) -> &mut Self {
        self.values = Some(values);
        self
    }
}

impl DB {
    // fn new() -> Self {
    //     DB
    // }

    pub fn new(connection: String) -> Self {
        DB {
            connection,
            command: None,
        }
    }

    fn command(&mut self, action: DBAction) -> &mut Self {
        self.command = Some(Command::new(action));
        self
    }

    fn fields(&mut self, fields: Vec<String>) -> &mut Self {
        if let Some(ref mut command) = self.command {
            command.fields(fields);
        }
        self
    }

    fn from(&mut self, table: String) -> &mut Self {
        if let Some(ref mut command) = self.command {
            command.table(table);
        }
        self
    }

    fn where_clause(&mut self, where_clause: String) -> &mut Self {
        if let Some(ref mut command) = self.command {
            command.where_clause(where_clause);
        }
        self
    }

    fn values(&mut self, values: Vec<String>) -> &mut Self {
        if let Some(ref mut command) = self.command {
            command.values(values);
        }
        self
    }

    fn execute(&self) {
        if let Some(ref command) = self.command {
            // println!("Executing command: {:?}", command);
        }
    }

    fn connection(&self) -> &String {
        &self.connection
    }
}
