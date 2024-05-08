## Project description

#### What is that I have to build?
You are tasked with setting up a backend server using Strapi and creating a WebSocket API that echoes client messages. The backend should facilitate real-time communication between clients and the server using WebSocket protocol.

#### What will my solution contain?

Backend Setup: Set up a backend server using Strapi, an open-source headless CMS, to provide a robust backend infrastructure for the application.

WebSocket API: Develop a WebSocket API that echoes client messages. When a client sends a message to the server, the server should send back the same message to the client. Ensure that the WebSocket connection is properly established and maintained throughout the communication process.

Database Integration: Integrate a database with Strapi to store relevant data, such as message history. You can use SQLite, PostgreSQL or another suitable database solution supported by Strapi.

Interface: While the primary focus is on backend development, you may create a simple interface or use tools like Postman to test the WebSocket API functionality and ensure proper message echoing.

Additional Features (Optional): Consider adding additional features to enhance the backend functionality, such as authorization, error handling, etc.



## My solution
The solution contains the following components:
The strapi backend server is set up to provide a robust backend infrastructure for the application.
i used the sqllite database to store the message history.
database contains a single model called Message with a single field called content.

#### Web sockets
The websockets is implemented in rust.
The `server/src/main.rs` file contain the code running the websocket server and saving messages into database.
The `server/index.html` file contains the frontend code, a small interface where the user can connect to the websocket server send messages , and it also shows the message history.


#### How to run

##### first clone the repo

```git

  cd ayna
  ## for strapi_backend 
  cd strapi_backend
  npm run develop
  ## for server
  cd server
  cargo run
  ## frontend
  cd server
  google-chrome index.html

```

