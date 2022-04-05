# APIs

---

Find all the API definitions for Kudo Scheduler including types, protocols and enumerations.

[External APIs](https://www.notion.so/External-APIs-47c5e284c00d412ca7751185a0ac4f3b)

## 💬 Types definition

---

### Structures

---

```protobuf
// Represents an Instance (eg. a container, VM ...)
message Instance {
    string id = 1;
    string name = 2;
    Type type = 3;
    State state = 4;
    string uri = 5;
    []string environnement = 6;
    Resource resource = 7;
    []string ports = 8;
}
```

```protobuf
// Represents a summary of all necessary resources
message ResourceSummary {
    int cpu = 1;
    int memory = 2;
    int disk = 3;
}

// Represent the maximum/usage of a Instance or a Node
message Resource {
    ResourceSummary max = 1;
    ResourceSummary usage = 2;
}
```

```protobuf
// Represents a Instance status message
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
    []Instance instances = 5;
}
```

```protobuf
// Represents a Instance action message
message InstanceAction {
    string id = 1;
    Action action = 2;
}
```

```protobuf
// Represents a Node Register request
message NodeRegisterRequest {
    string certificate = 1;
}

// Represents the response of the Node Register request
message NodeRegisterResponse {
    int code = 1;
    string description = 2;
}
```

### Enumerations

---

```protobuf
// Represents the Status of a node or a workflow
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
```

```protobuf
// Represents the different Instance actions possible
enum Action {
    START = 0;
    STOP = 1;
    DESTROY = 2;
    KILL = 3;
}
```

```protobuf
// Represents the different Type of a workflow
enum Type {
    CONTAINER = 0;
}
```



## ⚙️ Node → Scheduler (gRPC)

---

```protobuf
service AgentService {
    rpc NodeRegister (NodeRegisterRequest) returns (NodeRegisterResponse) {}
    rpc NodeStatus (google.protobuf.Empty) returns (stream NodeStatus) {}
    rpc InstanceStatus (google.protobuf.Empty) returns (stream InstanceStatus) {}
}
```

**NodeRegister** [...].

**NodeStatus** is a stream permits to send a `NodeStatus` object that contains metrics, all instances and the node status.

**InstanceStatus** is a stream permits to send a `InstanceStatus` object that contains the instance status.

## ⚙️ Controller → Scheduler (gRPC)

---

```protobuf
service ControllerService {
    rpc InstanceCreate (Instance) returns (google.protobuf.Empty) {}
    rpc InstanceUpdate (InstanceAction) returns (google.protobuf.Empty) {}
}
```

**InstanceCreate** are called when we want to launch a new instance to a `Node`. This call takes a `Instance` parameter including all the specification for the container runtime.

**InstanceUpdate** is used when we need to change the status of a `Instance`. This call takes a `Action` parameter (eg. stop, kill ...)
