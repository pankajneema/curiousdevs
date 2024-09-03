import asyncio
import websockets

# List to store active connections
connected_clients = []

# Handler for the WebSocket connection
async def handler(websocket, path):
    # Add the new client connection to the list
    print(f"New Connection Request :: {websocket}  ,path:{path}")
    connected_clients.append(websocket)
    print(f"TOtal Connected Clients :; {connected_clients}")
    try:
        async for message in websocket:
            print(f"Received message: {message}")
            # Broadcast the received message to all connected clients
            await broadcast(message)
    except websockets.exceptions.ConnectionClosed as e:
        print(f"Connection closed: {e}")
    finally:
        # Remove the client connection from the list when it disconnects
        connected_clients.remove(websocket)

# Function to broadcast a message to all connected clients
async def broadcast(message):
    if connected_clients:
        await asyncio.wait([client.send(message) for client in connected_clients])

# Start the WebSocket server
async def main():
    async with websockets.serve(handler, "127.0.0.1", 8080):
        print("WebSocket server started on ws://127.0.0.1:8080")
        await asyncio.Future()  # Run forever

# Run the server
if __name__ == "__main__":
    asyncio.run(main())

