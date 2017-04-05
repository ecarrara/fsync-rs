error_chain! {
    errors {
        InvalidUsage(msg: String) {
            description("invalid usage")
            display("invalid usage: {}", msg)
        }
    }

    foreign_links {
        Io(::std::io::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }
}
