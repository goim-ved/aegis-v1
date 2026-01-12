const axios = require('axios');

async function testLogin() {
    try {
        console.log("Attempting login with admin / password123 ...");
        const res = await axios.post('http://localhost:8080/api/auth/login', {
            username: 'admin',
            password: 'password123'
        });
        console.log("Login Success!");
        console.log("Token:", res.data.token);
    } catch (err) {
        console.error("Login Failed:");
        if (err.response) {
            console.error("Status:", err.response.status);
            console.error("Data:", err.response.data);
        } else {
            console.error(err.message);
        }
    }
}

testLogin();
