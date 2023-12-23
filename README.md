## Implementation Features

### Consistency
When interfacing from an arbitrary node, the data always be identical. 

### Data Replication
In the event of network partition, data replication makes this system 
fault tolerant

## API

### GET
`curl -X GET http://localhost:8080/store/your_key`
### PUT
`curl -X PUT -H "Content-Type: application/json" -d '{"value":"your_value"}'http://localhost:8080/store/your_key`
### DELETE
`curl -X DELETE http://localhost:8080/store/your_key`