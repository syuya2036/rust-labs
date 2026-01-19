#[tokio::main]
async fn main() {
    let (logger, guard) = LogWriter::new("app.log");

    for i in 0..2000 {
        tokio::spawn(async move { logerr.log(format!("INFO: from thread {}", i)) });
    }
}
