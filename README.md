# CSV Share Hub

This is a Rust web application designed for uploading and sharing CSV files. 
The application provides a user-friendly interface to easily view and interact with the data contained within the uploaded CSV files. 
Users can effortlessly upload their CSV documents, which will be processed and presented in an organized manner, making it simpler to analyze and share important information.

## Features

- Upload CSV files and store them locally.
- Intuitive interface for viewing and interacting with CSV data.
- Easy sharing options for users to share their data with others.

## Getting Started
Follow the instructions below to set up and run the application.

### Backend

1. **Clone the repository**

   ```bash
   git clone git@github.com:Wesley-Arizio/csvsharehub.git
   cd csvsharehub
   ```
2. **Navigate to the backend directory**
   ```bash
   cd backend
   ```
3. **Create local database**

   this file should be created in the root directory
   ```bash
   touch database.db
   ```
4. **Configure environmental variables**
   Copy the `.env.example` file to create a new `.env` file, and update the values according to your preferences.
   ```bash
   cp .env.example .env
   ```
5. **Run the project**
   ```bash
   cargo build
   ```

By default, the backend will run at http://localhost:8080, you can change it using .env file.


### Wasm
This crate is the code that will be used on the frontend to sort the data.
To build this you have to:

1. **Naviate to the directory**
   ```bash
   cd wasm
   ```

2. **Install wasm-pack**
   ```bash
   cargo install wasm-pack
   ```

2. **Build the project**
   ```bash
   wasm-pack build --target web
   ```


### Frontend
To run this project you must have node js and npm installed locally, find more [here](https://nodejs.org/en/download/package-manager)

1. **Navigate to the frontend directory**
   ```bash
   cd frontend
   ```
2. **(Optional) Use wasm build in the front-end** <br />
In order to update the wasm code that will be run on the front-end, you must first build the wasm crate,
and then, move the pkg folder into the directory. <br />
**Note**: if there's any significant changes in the wasm code, it might need changes on the front-end as well,
for simplicity I put a wasm build in there.
   ```bash
   # current dir: csvsharehub/frontend
   mv ../wasm/pkg ./src/
   ```
2. **Run the project**
  ```bash
   npm run start
   ```

By default, the backend will run at http://localhost:3000, you can change it using .env file.