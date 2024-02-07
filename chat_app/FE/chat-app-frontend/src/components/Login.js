import React, { useState } from 'react';
import axios from 'axios';

function Login({ history }) {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');

    const handleLogin = async () => {
        try {
            const response = await axios.post('http://localhost:8080/login', { username, password });
            // Handle response and redirect to chat room
            history.push('/chat');
        } catch (error) {
            setError('Login failed. Please check your username and password.');
        }
    };

    return (
        <div>
            <input type="text" value={username} onChange={e => setUsername(e.target.value)} placeholder="Username" />
            <input type="password" value={password} onChange={e => setPassword(e.target.value)} placeholder="Password" />
            <button onClick={handleLogin}>Login</button>
            {error && <p>{error}</p>}
        </div>
    );
}

export default Login;
