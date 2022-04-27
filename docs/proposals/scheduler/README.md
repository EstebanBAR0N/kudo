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
    repeated string ports = 8;
    string ip = 9;
}
```

```protobuf
// Represents a summary of all necessary resources
message ResourceSummary {
    int32 cpu = 1;
    int32 memory = 2;
    int32 disk = 3;
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
    repeated Instance instances = 5;
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
    int32 code = 1;
    string description = 2;
    string subnet = 3;
}
```

## ⚙️ Node → Scheduler (gRPC)

---

```protobuf
service AgentService {
    rpc NodeRegister (NodeRegisterRequest) returns (NodeRegisterResponse) {}
    rpc NodeStatusUpdate (stream NodeStatus) returns (google.protobuf.Empty) {}
    rpc InstanceStatusUpdate (stream InstanceStatus) returns (google.protobuf.Empty) {}
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