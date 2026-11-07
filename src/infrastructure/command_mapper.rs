use crate::domain::command::Command;
use crate::infrastructure::resp::RespFrame;
use bytes::Bytes;

impl Command {
    pub fn from_frame(frame: RespFrame) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match frame {
            RespFrame::Array(value) => {
                if value.is_empty() {
                    return Err("Empty command received from client".into());
                }

                let cmd_name = match &value[0] {
                    RespFrame::BulkString(s) => String::from_utf8_lossy(s).to_uppercase(),
                    RespFrame::SimpleString(s) => s.to_uppercase(),
                    _ => return Err("Invalid command name in the RESP protocol".into()),
                };

                match cmd_name.as_str() {
                    "SET" => {
                        if value.len() != 3 {
                            return Err("Incorrect syntax for SET. Use: SET <key> <value>".into());
                        }

                        let key = extract_string(&value[1])?;
                        let val = extract_bytes(&value[2])?;

                        Ok(Command::Set { key, value: val })
                    }
                    "GET" => {
                        if value.len() != 2 {
                            return Err("Incorrect syntax for GET. Use: GET <key>".into());
                        }
                        let key = extract_string(&value[1])?;
                        Ok(Command::Get { key })
                    }
                    other => Ok(Command::Unknown(other.to_string())),
                }
            }
            _ => Err("The protocol requires that requests be sent as RESP arrays.".into()),
        }
    }
}

fn extract_string(frame: &RespFrame) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    match frame {
        RespFrame::BulkString(bytes) => Ok(String::from_utf8(bytes.to_vec())?),
        _ => Err("Expecting a string in Bulk String format".into()),
    }
}

fn extract_bytes(frame: &RespFrame) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    match frame {
        RespFrame::BulkString(bytes) => Ok(bytes.clone()),
        _ => Err("Esperado dados em formato BulkString".into()),
    }
}
