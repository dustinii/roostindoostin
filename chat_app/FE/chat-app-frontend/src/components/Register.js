import React, { useState } from 'react';
import axios from 'axios';

function Register({ history }) {
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');

    const handleRegister = async () => {
        try {
            const response = await axios.post('http://localhost:8080/register', { username, email, password });
            // Handle response and redirect, maybe to login page
            history.push('/login');
        } catch (error) {
            setError('Registration failed. Please try again.');
        }
    };

    return (
        <div>
            <input type="text" value={username} onChange={e => setUsername(e.target.value)} placeholder="Username" />
            <input type="email" value={email} onChange={e => setEmail(e.target.value)} placeholder="Email" />
            <input type="password" value={password} onChange={e => setPassword(e.target.value)} placeholder="Password" />
            <button onClick={handleRegister}>Register</button>
            {error && <p>{error}</p>}
        </div>
    );
}

export default Register;
