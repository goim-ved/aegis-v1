const axios = require('axios');

async function seedData() {
    try {
        // 1. Login
        console.log("Logging in as admin...");
        const loginRes = await axios.post('http://localhost:8080/api/auth/login', {
            username: 'admin',
            password: 'password123'
        });
        const token = loginRes.data.token;
        console.log("Login successful!");

        // 2. Register Entity
        console.log("Registering Test Entity 'Tesla Corp'...");
        const config = {
            headers: { Authorization: `Bearer ${token}` }
        };

        const entityRes = await axios.post('http://localhost:8080/api/compliance/register', {
            hash_id: "US_DEL_559922",
            jurisdiction: "Delaware, US",
            kyc_level: 3
        }, config);

        console.log("Entity Registered!", entityRes.data);
        console.log("\nSUCCESS! Refresh your dashboard to see the new entity.");

    } catch (err) {
        console.error("Seeding Failed:", err.message);
        if (err.response) console.error(err.response.data);
    }
}

seedData();
