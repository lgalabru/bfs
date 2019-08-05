
#[derive(Debug)]
struct ListFilesCommand {

    pub prefix_path: OsString, 
    
    pub page: Option<u32>
}

impl Command for ListFilesCommand {

    fn validate() {

    }

    fn run() {

    }
}
