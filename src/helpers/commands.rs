pub fn skip_sub_command(args: Vec<String>) -> Vec<String> {
    let args: Vec<String> = args.into_iter().skip(1).collect();
    args
}
