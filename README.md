# affirMe
An application for positive affirmations.

## 🎉 Running the project
Start the backend first in one terminal window, then start the frontend in another terminal window.

### 🌚 Backend
1. **Install MongoDB and Rust:** Ensure MongoDB and Rust are installed. Refer to [https://www.mongodb.com/try/download/community](https://www.mongodb.com/try/download/community) for MongoDB installation and [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for Rust installation.
2. **Start the MongoDB Server:** Run `mongod` in a terminal window.
3. **Configure MongoDB connection:** The backend is designed to connect to a local MongoDB database. The default connection details, including the URI, database name, and collection name, are stored in the *.env* file provided. Verify that the values in the *.env* file match your local MongoDB setup.
4. **Insert dataset:** In the backend folder, run `cargo run --bin insert_data` to insert the affirmations dataset into MongoDB.
5. **Launch the web server:** After step 4, run `cargo run --bin server` to start the web server on localhost, port 8080. Access the endpoint [http://localhost:8080/affirmations/random](http://localhost:8080/affirmations/random) for a random affirmation.

### 🌞 Frontend
1. **Install Node.js:** Download Node.js. Refer to [https://nodejs.org](https://nodejs.org) for installation instructions.
2. **Install dependencies:** In the frontend folder, run `pnpm i` to install the project dependencies listed in *package.json*.
3. **Start the development server:** After step 2, run `pnpm dev`. Access the application at [http://localhost:5173](http://localhost:5173) in your web browser.

*Note:* The instructions assume the usage of *pnpm* as the package manager. If you prefer *npm*, replace *pnpm* with *npm*. To install *pnpm*, run `npm install -g pnpm` in a terminal window.
