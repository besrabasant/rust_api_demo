# Rust API Demo


Practicing Rust

## Usage

### Setting Up Environment Variables
To run this Rust API project, you need to set up environment variables. This is typically done using a `.env` file. Below are the steps to copy and create a new `.env` file for your project:

1. **Copy the Example `.env` File**:
    - Copy the `.env.example` file to create a new `.env` file. This can be done using the following command in your terminal:
 
    ```sh
    cp .env.example .env
    ```

2. **Edit the `.env` File**:
    - Open the newly created `.env` file in your preferred text editor.
    - Update the values of the environment variables as needed for your local setup. The `.env` file might look something like this:
    ```dotenv
    APP_PORT=8000 # or any custom port
    ```