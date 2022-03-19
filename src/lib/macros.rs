#[macro_export]
macro_rules! flush_stdout {
    () => {{
        io::stdout()
            .flush()
            .with_context(|| "Failed to flush stdout")?;
    }};
}
