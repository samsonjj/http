use http;

fn main() -> std::io::Result<()> {
    http::listen(8080)?;
    Ok(())
}