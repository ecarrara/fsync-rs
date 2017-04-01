error_chain! {
    foreign_links {
        Io(::std::io::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }
}
