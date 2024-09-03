// "use client"
// // import React from "react";



// // const DashboardPage = () => {
// //     return (
// //         <div>
// //             <h1>Da</h1>
// //         </div>
// //     );
// // };

// // export default DashboardPage;
// // // pages/index.tsx

// import { useEffect, useState } from 'react';
// import { initWebSocket } from '../socket/wsListener';

// interface Message {
//   content: string;
// }

// export default function Home() {
//   const [messages, setMessages] = useState<Message[]>([]);

//   useEffect(() => {
//     const socket = initWebSocket('ws://127.0.0.1:8080');

//     // Set the onmessage handler directly
//     socket.onmessage = (event: MessageEvent) => {
//         console.log("Event Data :",event);
//         const messagesDiv = document.getElementById('messages');
//         const messageElement = document.createElement('p');
//         messageElement.textContent = `Received: ${event.data}`;
//         messagesDiv.appendChild(messageElement);
//     };

//     // Cleanup on component unmount
//     return () => {
//       socket.onmessage = null; // Remove the onmessage handler
//     };
//   }, []);

//   return (
//     <div>
//       <h1>WebSocket Messages</h1>
//       <ul>
//       <div id="messages"></div>
//       </ul>
//     </div>
//   );
// }
"use client";
import { useEffect, useState } from 'react';
import { initWebSocket } from '../socket/wsListener';

interface Message {
  content: string;
}

export default function Home() {
  const [messages, setMessages] = useState<string[]>([]); // Store messages as an array of strings

  useEffect(() => {
    const socket = initWebSocket('ws://127.0.0.1:8080/ws');

    // Handle incoming WebSocket messages
    socket.onmessage = (event: MessageEvent) => {
      console.log("Received Event Data:", event.data);
      setMessages(prevMessages => [...prevMessages, event.data]); // Add new message to the state
    };

    // Cleanup on component unmount
    return () => {
        socket.onmessage = null; // Close the WebSocket connection
    };
  }, []);

  return (
    <div>
      <h1>WebSocket Messages</h1>
      <div id="messages">
        {messages.map(message => (
          <p key={message}>Received: {message}</p>
        ))}
      </div>
    </div>
  );
}
