//! # ddaa_protocol: The Ding Ding Ack Ack Protocol
//!
//! `ddaa_protocol` is a crate for parsing and formatting Ding Ding Ack Ack protocol messages.
///```
/// // Test read request.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x07;
/// buffer[1] = 0x01;
/// buffer[2] = 0x05;
/// buffer[3] = 0x00;
/// buffer[4] = 0x01;
/// buffer[5] = 0x00;
/// buffer[6] = 0x17;
/// buffer[7] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::Request,
///     command: Command::Read,
///     variable: 0x00,
///     data: vec![0x00],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Request, Command: Read, Variable: 0x00, Data: [0x00]"
/// );
/// ```
/// ```
/// // Test write request.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x07;
/// buffer[1] = 0x01;
/// buffer[2] = 0x1A;
/// buffer[3] = 0x00;
/// buffer[4] = 0x01;
/// buffer[5] = 0x00;
/// buffer[6] = 0x17;
/// buffer[7] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::Request,
///     command: Command::Write,
///     variable: 0x00,
///     data: vec![0x00],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Request, Command: Write, Variable: 0x00, Data: [0x00]"
/// );
/// ```
/// ```
/// // Test ping request.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x07;
/// buffer[1] = 0x01;
/// buffer[2] = 0x07;
/// buffer[3] = 0x00;
/// buffer[4] = 0x01;
/// buffer[5] = 0x00;
/// buffer[6] = 0x17;
/// buffer[7] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::Request,
///     command: Command::Ping,
///     variable: 0x00,
///     data: vec![0x00],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Request, Command: Ping, Variable: 0x00, Data: [0x00]"
/// );
/// ```
/// ```
/// // Test response success.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x06;
/// buffer[1] = 0x01;
/// buffer[2] = 0x05;
/// buffer[3] = 0x00;
/// buffer[4] = 0x01;
/// buffer[5] = 0x00;
/// buffer[6] = 0x17;
/// buffer[7] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::ResponseSuccess,
///     command: Command::Read,
///     variable: 0x00,
///     data: vec![0x00],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Response Success, Command: Read, Variable: 0x00, Data: [0x00]"
/// );
/// ```
/// ```
/// // Test response error.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x15;
/// buffer[1] = 0x01;
/// buffer[2] = 0x05;
/// buffer[3] = 0x00;
/// buffer[4] = 0x01;
/// buffer[5] = 0x00;
/// buffer[6] = 0x17;
/// buffer[7] = 0x04;
/// buffer[8] = 0x01;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::ResponseError,
///     command: Command::Read,
///     variable: 0x00,
///     data: vec![0x00],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Response Error, Command: Read, Variable: 0x00, Data: [0x00]"
/// );
/// ```
/// ```
/// // Test ping response.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x06;
/// buffer[1] = 0x01;
/// buffer[2] = 0x05;
/// buffer[3] = 0x00;
/// buffer[4] = 0x01;
/// buffer[5] = 0x00;
/// buffer[6] = 0x17;
/// buffer[7] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::ResponseSuccess,
///     command: Command::Read,
///     variable: 0x00,
///     data: vec![0x00],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Response Success, Command: Read, Variable: 0x00, Data: [0x00]"
/// );
/// ```
/// ```
/// // Test Multibyte Request
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x07;
/// buffer[1] = 0x01;
/// buffer[2] = 0x1A;
/// buffer[3] = 0x02;
/// buffer[4] = 0x03;
/// buffer[5] = 0x00;
/// buffer[6] = 0x01;
/// buffer[7] = 0x02;
/// buffer[8] = 0x17;
/// buffer[9] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::Request,
///     command: Command::Write,
///     variable: 0x02,
///     data: vec![0x00, 0x01, 0x02],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Request, Command: write, Variable: 0x02, Data: [0x00, 0x01, 0x02]"
/// );
/// ```
/// ```
/// // Test Multibyte Response
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x06;
/// buffer[1] = 0x01;
/// buffer[2] = 0x05;
/// buffer[3] = 0x02;
/// buffer[4] = 0x03;
/// buffer[5] = 0x00;
/// buffer[6] = 0x01;
/// buffer[7] = 0x02;
/// buffer[8] = 0x17;
/// buffer[9] = 0x04;
///
/// let expected = ProtocolMessage {
///     message_type: MessageType::ResponseSuccess,
///     command: Command::Read,
///     variable: 0x02,
///     data: vec![0x00, 0x01, 0x02],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Response Success, Command: read, Variable: 0x02, Data: [0x00, 0x01, 0x02]"
/// );
/// ```
/// ```
/// // Test data shifting (buffer is big, and doesn't have a valid set of start bytes.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut buffer: Vec<u8> = vec![0; 38];
/// buffer[0] = 0x01;
/// buffer[1] = 0x05;
/// buffer[2] = 0x02;
/// buffer[3] = 0x03;
/// buffer[4] = 0x00;
/// buffer[5] = 0x01;
/// buffer[6] = 0x02;
/// buffer[7] = 0x17;
/// buffer[8] = 0x04;
/// buffer[9] = 0x06;
/// buffer[10] = 0x01;
/// buffer[11] = 0x05;
/// buffer[12] = 0x02;
/// buffer[13] = 0x03;
/// buffer[14] = 0x00;
/// buffer[15] = 0x01;
/// buffer[16] = 0x02;
/// buffer[17] = 0x17;
/// buffer[18] = 0x04;
/// buffer[19] = 0x06;
/// buffer[20] = 0x01;
/// buffer[21] = 0x05;
/// buffer[22] = 0x02;
/// buffer[23] = 0x03;
/// let expected = ProtocolMessage {
///     message_type: MessageType::ResponseSuccess,
///     command: Command::Read,
///     variable: 0x02,
///     data: vec![0x00, 0x01, 0x02],
/// };
/// assert_eq!(
///     expected,
///     ddaa_protocol::parse_protocol_message(&mut buffer).unwrap(),
///     "Type: Response Success, Command: read, Variable: 0x02, Data: [0x00, 0x01, 0x02]"
/// );
/// ```
/// ```
/// // Test create read request
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut expected_buffer: Vec<u8> = vec![0; 10];
/// expected_buffer[0] = 0x07;
/// expected_buffer[1] = 0x01;
/// expected_buffer[2] = 0x05;
/// expected_buffer[3] = 0x01;
/// expected_buffer[4] = 0x03;
/// expected_buffer[5] = 0x01;
/// expected_buffer[6] = 0x03;
/// expected_buffer[7] = 0x02;
/// expected_buffer[8] = 0x17;
/// expected_buffer[9] = 0x04;
///
/// let mut data: Vec<u8> = vec![0; 3];
/// data[0] = 0x01;
/// data[1] = 0x03;
/// data[2] = 0x02;
///
/// assert_eq!(
///    expected_buffer,
///    ddaa_protocol::create_protocol_buffer(
///        MessageType::Request,
///        Command::Read,
///        0x01,
///        &data
///    ),
///    "Create Type: Request, Command: Read, Variable: 0x01, Data: [0x01 0x03 0x02]"
/// );
/// ```
/// ```
/// // Test create write request.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut expected_buffer: Vec<u8> = vec![0; 10];
/// expected_buffer[0] = 0x07;
/// expected_buffer[1] = 0x01;
/// expected_buffer[2] = 0x1A;
/// expected_buffer[3] = 0x01;
/// expected_buffer[4] = 0x03;
/// expected_buffer[5] = 0x01;
/// expected_buffer[6] = 0x03;
/// expected_buffer[7] = 0x02;
/// expected_buffer[8] = 0x17;
/// expected_buffer[9] = 0x04;
///
/// let mut data: Vec<u8> = vec![0; 3];
/// data[0] = 0x01;
/// data[1] = 0x03;
/// data[2] = 0x02;
/// assert_eq!(
///    expected_buffer,
///    ddaa_protocol::create_protocol_buffer(
///        MessageType::Request,
///        Command::Write,
///        0x01,
///        &data
///    ),
///    "Create Type: Request, Command: Write, Variable: 0x01, Data: [0x01 0x03 0x01]"
/// );
/// ```
/// ```
/// // Test create ping request.
/// use ddaa_protocol::{Command, MessageType, ProtocolMessage};
/// let mut expected_buffer: Vec<u8> = vec![0; 8];
/// expected_buffer[0] = 0x07;
/// expected_buffer[1] = 0x01;
/// expected_buffer[2] = 0x07;
/// expected_buffer[3] = 0x01;
/// expected_buffer[4] = 0x01;
/// expected_buffer[5] = 0x01;
/// expected_buffer[6] = 0x17;
/// expected_buffer[7] = 0x04;
///
/// let mut data: Vec<u8> = vec![0; 1];
/// data[0] = 0x01;
/// assert_eq!(
///    expected_buffer,
///    ddaa_protocol::create_protocol_buffer(
///        MessageType::Request,
///        Command::Ping,
///        0x01,
///        &data
///    ),
///    "Create Type: Request, Command: Ping, Variable: 0x01, Data: [0x01]"
/// );
/// ```
// Define an enum for message types
#[derive(Debug, PartialEq, Eq)]
pub enum MessageType {
    Request,
    ResponseSuccess,
    ResponseError,
}

// Define an enum for commands
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Read,
    Write,
    Ping,
}

// Define a struct to represent the parsed message
#[derive(Debug, PartialEq, Eq)]
pub struct ProtocolMessage {
    pub message_type: MessageType,
    pub command: Command,
    pub variable: u8,
    pub data: Vec<u8>,
}

// Create a buffer for sending a protocol message
pub fn create_protocol_buffer(
    message_type: MessageType,
    command: Command,
    variable: u8,
    data: &[u8]
) -> Vec<u8> {
    let data_len = data.len().min(32); // Limit data length to maximum 32 bytes
    let mut buffer: Vec<u8> = Vec::with_capacity(7 + data_len);

    // Add protocol message fields to the buffer
    buffer.push(match message_type {
        MessageType::Request => 0x07,
        MessageType::ResponseSuccess => 0x06,
        MessageType::ResponseError => 0x15,
    });
    buffer.push(0x01);
    buffer.push(match command {
        Command::Read => 0x05,
        Command::Write => 0x1A,
        Command::Ping => 0x07,
    });
    buffer.push(variable);
    buffer.push(data_len as u8);
    buffer.extend_from_slice(&data[0..data_len]);
    buffer.push(0x17);
    buffer.push(0x04);

    buffer
}

// Implement a function to parse the protocol message
pub fn parse_protocol_message(data: &mut Vec<u8>) -> Option<ProtocolMessage> {
    loop {
        if data.len() < 8 {
            return None; // Incomplete message, return None.
        }

        if (data[0] == 0x07 || data[0] == 0x06) && data[1] == 0x01 {
            // Vector shifted to correct starting position.
            break;
        }

        Some(data.remove(0));
    }

    // Shift the bytes, till we get a correct message.
    let message_type = match data[0] {
        0x07 => MessageType::Request,
        0x06 => MessageType::ResponseSuccess,
        0x15 => MessageType::ResponseError,
        _ => return None, // Invalid message type, return None.
    };

    let command = match data[2] {
        0x05 => Command::Read,
        0x1A => Command::Write,
        0x07 => Command::Ping,
        _ => return None, // Invalid command, return None.
    };

    let variable = data[3];
    let data_length = data[4] as usize;

    if data.len() < 7 + data_length
        || data[data_length + 5] != 0x17
        || data[data_length + 6] != 0x04
    {
        return None; // Invalid message, return None
    }

    let message = ProtocolMessage {
        message_type,
        command,
        variable,
        data: data[5..5 + data_length].to_vec(),
    };

    Some(message)
}
