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

1. **Clone the repository**

   ```bash
   git clone git@github.com:Wesley-Arizio/csvsharehub.git
   cd csvsharehub
   ```
2. **Create local database**
   
   ```bash
   touch database.db
   ```
3. **Configure environmental variables**
   Copy the `.env.example` file to create a new `.env` file, and update the values according to your preferences.
   ```bash
   cp .env.example .env
   ```
4. **Run the migrations**
   To run this command, you'll need to install diesel cli. [Instructions here](https://github.com/diesel-rs/diesel/tree/2.2.x/diesel_cli)
   ```bash
   diesel migration run --database-url=database.db
   ```
2. **Run the project**
   ```bash
   cargo build
   ```
