use crate::domain::command::Command;
use crate::domain::repository::DatabaseRepository;
use crate::infrastructure::resp::RespFrame;
use crate::use_cases::set_value::SetValueUseCase;
use std::sync::Arc;

async fn process_command(
    frame: RespFrame,
    db: Arc<dyn DatabaseRepository>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let command = Command::from_frame(frame)?;

    match command {
        Command::Set { key, value } => {
            let use_case = SetValueUseCase::new(db);

            use_case.execute(key, value).await;
            println!("SET command processed successfully.");
        }
        Command::Get { key } => {
            println!("GET command received for the key: {}", key);
        }
        Command::Unknown(cmd) => {
            eprintln!("Unknown command received: {}", cmd);
        }
    }

    Ok(())
}
