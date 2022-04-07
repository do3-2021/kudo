# External API

---

## Routes

### /instance/

| Method/Route | Description                   | Parameters                 |
| ------------ | ----------------------------- | -------------------------- |
| GET /        | get a list of instances       | limit, offset, type, state |
| GET /{id}    | get detailled info on instance | instanceId             |
| PUT /        | create an instance            |                            |
| PATCH /{id}  | update an instance            | instanceId                 |
| DELETE /{id} | delete an instance            | instanceId                 |

### /workload/

| Method/Route | Description                   | Parameters          |
| ------------ | ----------------------------- | ------------------- |
| GET /        | get a list of workloads       | limit, offset, type |
| GET /{id}    | get detailled info on workload | workloadId       |
| PUT /        | create a workload             |                     |
| PATCH /{id}  | update a workload             | workloadId         |
| DELETE /{id} | delete a workload             | workloadId         |

## External Structures

### Instance

```rust
struct Instance {
    id: String,
    name: String,
    type: Type,
    status: Status,
    uri: String,
    environment: [String, 100],
    resources: Resources,
    ports: [String, 100]
}
```

### Workload

```rust
struct Workload {
    id: String,
    name: String,
    type: Type,
    uri: String,
    environment: [String, 100],
    resources: Resources,
    ports: [String, 100]
}
```

### Type

```rust
enum Type {
    CONTAINER
}
```

### State

```rust
enum Status {
    RUNNING,
    STARTING,
    STOPPED,
    STOPPING,
    DESTROYING,
    TERMINATED,
    CRASHED,
    FAILED,
    SCHEDULING,
    SCHEDULED
}
```

### Resources

```rust
struct Resource {
    cpu: i32,
    memory: i32,
    disk: i32
}
```

```rust
struct ResourceClaim {
    max: Resource,
    usage: Resource
}
```

# Internal API

---

## Internal Structures

### Messages

```protobuf
// Represents an Instance status message
message InstanceStatus {
    string id = 1;
    Status status = 2;
    string description = 3;
}

// Represents a Node status message
message NodeStatus {
    string id = 1;
    Status status = 2;
    string description = 3;
    Resource resource = 4;
    [] Instance instances = 5;
}

//Represents an Instance running
message Instance {
    string id = 1;
    string name = 2;
    Type type = 3;
    Status status = 4;
    string uri = 5;
    [] string environnement = 6;
    Resource resource = 7;
    [] string ports = 8;
    string ip = 9;
}

//Represents a Resource entry
message ResourceSummary {
    int cpu = 1;
    int memory = 2;
    int disk = 3;
}

//Represents the resources used & threshold
message Resource {
    ResourceSummary max = 1;
    ResourceSummary usage = 2;
}
```

### Enums

```protobuf
//Represents the liveness status of
enum Status {
    RUNNING = 0;
    STARTING = 1;
    STOPPED = 2;
    STOPPING = 3;
    DESTROYING = 4;
    TERMINATED = 5;
    CRASHED = 6;
    FAILED = 7;
    SCHEDULING = 8;
    SCHEDULED = 9;
}

//Represents the type of workload running
enum Type {
    CONTAINER = 0;
}
```

## Functions

### Controller → Scheduler (gRPC)

```protobuf
service SchedulerService {
    rpc setNodeStatus(NodeStatus) returns (google.protobuf.Empty) {}
    rpc setInstanceStatus(InstanceStatus) returns (google.protobuf.Empty) {}
}

```
