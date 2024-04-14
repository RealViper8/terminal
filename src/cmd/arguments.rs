pub fn parse(args: Vec<String>) -> Result<Vec<String>, String> {
    let mut arguments: Vec<String> = vec![];
    for arg in args.iter() {
        if arg.starts_with('-') {
            arguments.push(arg.replace('-', "").to_lowercase());
        }
    }

    Ok(arguments)
}
