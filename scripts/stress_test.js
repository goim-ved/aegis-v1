const axios = require('axios');

const API_URL = 'http://localhost:8080/api';
const CONCURRENCY = 50;

async function runTest() {
    console.log('Starting Stress Test...');

    // 1. Authenticate
    let token;
    try {
        // Try login or register if login fails (idempotent-ish)
        try {
            const res = await axios.post(`${API_URL}/auth/login`, { username: 'stress_tester', password: 'password123' });
            token = res.data.token;
        } catch {
            await axios.post(`${API_URL}/auth/register`, { username: 'stress_tester', password: 'password123' });
            const res = await axios.post(`${API_URL}/auth/login`, { username: 'stress_tester', password: 'password123' });
            token = res.data.token;
        }
        console.log('Authentication Successful.');
    } catch (err) {
        console.error('Auth Failed:', err.message);
        process.exit(1);
    }

    // 2. Concurrent Requests
    console.log(`Firing ${CONCURRENCY} concurrent requests to /compliance/entities...`);
    const start = Date.now();

    const requests = Array.from({ length: CONCURRENCY }).map(() =>
        axios.get(`${API_URL}/compliance/entities`, {
            headers: { Authorization: `Bearer ${token}` }
        })
    );

    try {
        await Promise.all(requests);
        const duration = Date.now() - start;
        console.log(`\n\u2705 Success! Processed ${CONCURRENCY} requests in ${duration}ms.`);
        console.log(`Average Latency: ${(duration / CONCURRENCY).toFixed(2)}ms`);
        console.log(`Throughput: ${(CONCURRENCY / (duration / 1000)).toFixed(2)} req/s`);
    } catch (err) {
        console.error('\u274C Stress Test Failed:', err.message);
    }
}

runTest();
