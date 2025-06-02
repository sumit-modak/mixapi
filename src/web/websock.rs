use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
};

// GET /path/to/route HTTP/1.1
// Host: server.example.com
// Upgrade: websocket
// Connection: Upgrade
// Sec-Websocket-Key: x3JJHMbDL1EzLkh9GBhXDw==
// Sec-Websocket-Prococol: chat, superchat
// Sec-Websocket-Version: 13
// Origin: http://example.com

// HTTP/1.1 101 SWITCHING PROTOCOLS
// Upgrade: websocket
// Connection: Upgrade
// Sec-Websocket-Accept: HSmrc0sMlYUkAGmm5OPpG2HaGWk=
// Sec-Websocket-Prococol: chat

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("{ws:#?}");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        // if let Some(msg) = socket.recv().await {
        //     let msg = if let Ok(msg) = msg {
        //         msg
        //     } else {
        //         // client disconnected
        //         return;
        //     };

        //     if socket.send(msg).await.is_err() {
        //         // client disconnected
        //         return;
        //     }
        // }
        tokio::time::sleep(tokio::time::Duration::new(2, 0)).await;
        if socket
            .send(Message::Text("what's up?".into()))
            .await
            .is_err()
        {
            // client disconnected
            return;
        }
    }
    // if None
    // that means the stream is closed
}

// Handler for WebSocket connection upgrades
pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("{ws:#?}");
    ws.on_upgrade(handle_websocket)
}

// Function to handle the WebSocket connection
async fn handle_websocket(mut socket: WebSocket) {
    println!("WebSocket connection established");

    // Use a loop to handle messages from the client
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                println!("Received text message: {}", text);

                // Echo the message back to the client
                if socket
                    .send(Message::Text(format!("Echo: {}", text).into()))
                    .await
                    .is_err()
                {
                    println!("Failed to send message");
                    return;
                }
            }
            Message::Binary(bin) => {
                println!("Received binary message: {:?}", bin);

                // Echo the binary message back to the client
                if socket.send(Message::Binary(bin)).await.is_err() {
                    println!("Failed to send binary message");
                    return;
                }
            }
            Message::Close(_) => {
                println!("Client disconnected");
                break;
            }
            _ => {
                println!("Unhandled message type");
            }
        }
    }
    println!("WebSocket connection closed");
}

// Setting up client on browser

// let connection = new WebSocket("ws://localhost:3000/websock");
// connection.onmessage = console.log;
// connection.send("hi");

// Setting up client using wscat

// wscat -c ws://127.0.0.1:3000/websock2
