# Copy service

This service synchronize information between two directories.
The idea of this service is to coping information in a controlled way allowing to recuperate the copy in case the connection is down.

## Start service

tbd

## Protocol

|           | size    | msg id  | flags  | addr    | port    | msg           |
|-----------|---------|---------|--------|---------|---------|---------------|
| size      | 2 bytes | 2 bytes | 1 byte | 4 bytes | 2 bytes | max 500 bytes |
| bytes     | 0       | 00      | 0      | 0000    | 00      | 0...498...0   |
| start pos | 0       | 1       | 3      | 4       | 8       | 10            |

**Protocol description**

- **size**: Message size, starting in the flag and ending in the last message character.
- **msg id**: Message identification for the client.
- **flags**: 8 bits flags to pass extra information.
- **addr**: 4 bytes defining ipv4 destination address.
- **port**: 2 bytes defining the destination port.
- **msg**: The message with not more than 512 byte length.

## License

tbd

## Developer

- Guille

## ToDo List

- Connect master and slave services
- Define the communication protocol
- Integrate rocket and create simple api to controlled the files (remover, coping, moving, etc) between the services.
- Investigate and integrate a library for managing the application args.

