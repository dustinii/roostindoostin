import React, { useState, useEffect, useRef } from 'react';

function ChatRoom() {
    const [message, setMessage] = useState('');
    const [messages, setMessages] = useState([]);
    const ws = useRef(null);

    useEffect(() => {
        ws.current = new WebSocket('ws://localhost:8080/ws/');
        ws.current.onmessage = event => {
            const newMessage = JSON.parse(event.data);
            setMessages(prevMessages => [...prevMessages, newMessage]);
        };

        return () => {
            if (ws.current) {
                ws.current.close();
            }
        };
    }, []);

    const sendMessage = () => {
        if (message && ws.current) {
            ws.current.send(JSON.stringify({ content: message }));
            setMessage('');
        }
    };

    return (
        <div>
            <input type="text" value={message} onChange={e => setMessage(e.target.value)} />
            <button onClick={sendMessage}>Send</button>
            <div>
                {messages.map((msg, index) => (
                    <p key={index}>{msg.content}</p>
                ))}
            </div>
        </div>
    );
}

export default ChatRoom;
