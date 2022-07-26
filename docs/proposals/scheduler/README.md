# APIs

---

Find all the API definitions for Kudo Scheduler including types, protocols and enumerations.

[External APIs](https://www.notion.so/External-APIs-47c5e284c00d412ca7751185a0ac4f3b)

## 💬 Types definition

---

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
    FAILED = 6;
    SCHEDULING = 7;
    SCHEDULED = 8;
}
```

```protobuf
// Represents the different Type of a workflow
enum Type {
    CONTAINER = 0;
}
```

### Structures

---

```protobuf
// Represents an Instance (eg. a container, VM ...)
message Instance {
    string id = 1;
    string name = 2;
    Type type = 3;
    Status status = 4;
    string uri = 5;
    repeated string environnement = 6;
    Resource resource = 7;
    repeated Port ports = 8;
    string ip = 9;
}
```

```protobuf
// Represents a Port
message Port {
    int32 source = 1;
    int32 destination = 2;
}
```

```protobuf
// Represents a summary of all necessary resources
message ResourceSummary {
    int32 cpu = 1;
    int32 memory = 2;
    int32 disk = 3;
}

// Represent the limit/usage of a Instance or a Node
message Resource {
    ResourceSummary limit = 1; // cpu, memory, disk limit for a workload or a node
    ResourceSummary usage = 2; // cpu, memory, disk usage in real-time for a workload or a node
}
```

```protobuf
// Represents a Instance status message
message InstanceStatus {
    string id = 1;
    Status status = 2;
    string statusDescription = 3;
    Resource resource = 4;
}

// Represents a Node status message
message NodeStatus {
    string id = 1;
    Status status = 2;
    string statusDescription = 3;
    Resource resource = 4;
}
```

```protobuf
// Represents a Node Register request
message NodeRegisterRequest {
    string certificate = 1;
}

// Represents the response of the Node Register request
message NodeRegisterResponse {
    int32 code = 1;
    string description = 2;
    string subnet = 3;
}

message NodeUnregisterRequest {
  string id = 1;
}

message NodeUnregisterResponse {
  int32 code = 1;
  string description = 2;
}

message InstanceIdentifier {
    string id = 1;
}
```

## ⚙️ Node → Scheduler (gRPC)

---

```protobuf
service NodeService {
    rpc Status (stream NodeStatus) returns (google.protobuf.Empty) {}
    rpc Register (NodeRegisterRequest) returns (NodeRegisterResponse) {}
    rpc Unregister (NodeUnregisterRequest) returns (NodeUnregisterResponse) {}
}
```

**Register** [...].

**Unregister** [...].

## ⚙️ Controller → Scheduler (gRPC)

---

```protobuf
service InstanceService {
    rpc Create (Instance) returns (stream InstanceStatus) {}
    rpc Start (InstanceIdentifier) returns (google.protobuf.Empty) {}
    rpc Stop (InstanceIdentifier) returns (google.protobuf.Empty) {}
    rpc Destroy (InstanceIdentifier) returns (google.protobuf.Empty) {}
}
```

**Create** are called when we want to launch a new instance to a `Node`. This call takes a `Instance` parameter including all the specification for the container runtime and returns a stream of all the instance's updates.

**Start** are called to start an instance. This call takes a `string` parameter
for the instance id.

**Stop** are called to stop an instance. This call takes a `string` parameter for the
instance id.

**Destroy** are called to destroy an instance. This call takes a `string` parameter for the instance id.
