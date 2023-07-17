# affirMe
An application for getting and creating positive affirmations.

## 🎉 Running the project
To run the project, start the backend first in one terminal window and then start the frontend in another terminal window.

### 🍑 Backend
1. **Install MongoDB and Rust**: Ensure that MongoDB and Rust are installed on your system. To install MongoDB, visit [https://www.mongodb.com/try/download/community](https://www.mongodb.com/try/download/community) and follow the instructions for your operating system. For Rust installation, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the provided instructions.
2. **Start the MongoDB Server:** Open a terminal prompt an run `mongod` to start the MongoDB server.
3. **Configure the MongoDB connection:** The backend is designed to connect to a local MongoDB database. The default connection details, including the URI, database name, and collection name, are stored in the `.env` file provided. Make sure to verify that the values in the `.env` file match your local MongoDB setup.
4. **Insert dataset:** Open a terminal window, navigate to the backend folder, and run `cargo run --bin insert_data`. This binary will insert the affirmations dataset into your MongoDB database.
5. **Launch the web server:** Once step 4 has completed successfully, execute the command `cargo run --bin backend` to start the web server. If everything went smoothly, the server will be up and running on localhost, port 8080. Now, you can access a delightful random affirmation by visiting the endpoint [http://localhost:8080/affirmations/random](http://localhost:8080/affirmations/random).

### 🍒 Frontend
1. **Install Node.js:** Make sure you have Node.js installed on your system. You can download it from [https://nodejs.org](https://nodejs.org) and follow the installation instructions for your operating system.
   - Note: The instructions assume the usage of `pnpm` as the package manager. If you prefer to use `npm`, you can replace `pnpm` with `npm`. If you don't have `pnpm` installed and want to use it, you can install it by running `npm install -g pnpm` in a terminal window.
2. **Install dependencies:** Open a terminal window, navigate to the frontend folder, and run `pnpm i` to install the project dependencies listed in the `package.json` file.
3. **Start the development server:** Once step 2 has completed successfully, run `pnpm run dev` in the terminal. This command will start the development server, and you can access the application by visiting [http://localhost:5173](http://localhost:5173) in your web browser.

<br><br>

<img src="./img/cat.png" width="60%">
