use gitlaw::std_error_exit;

pub fn help(title: String) {
    std_error_exit!(format!(
        "{}
        OPTIONS:\n  \
               -e,              Download the Gitlaw engine\n  \
               -m,              Download the AI model\n  \
               -h, --help       Show this mystical guide",
        title
    ));
}