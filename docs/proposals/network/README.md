# Network

## How it works

### Introduction

The objective of the network service is to make several instances communicate with each other as if they were all connected to a single private network, no matter if they are on the same machine or not.

### Suggested approach

To try to solve this problem, the network service is responsible for creating, configuring and deleting virtual network interfaces used to connect instances to each other within a cluster.

This service has to be running on all nodes of the cluster.

When adding a new node to a cluster, a call to the network service must be done to set up a new container network interface (CNI) service. There is one CNI service per node, and that service bridges the networking interfaces associated with the instances running on the node it controls to the rest of the cluster.

A call to the network service must also be made before creating a new instance in order to set up the virtual network interface connecting the instance to a CNI.

After the destruction of an instance, the network service must be called to remove the network interface associated with it and reconfigure the routing tables.

![Node example](schema.png)

## API

### Messages

```protobuf
// Request structure used to create a new virtual network interface
message CreateNetworkInterfaceRequest {
    string workload_id = 1;
    string ip_address = 2;
    repeated int32 ports = 3;
}

// Response structure returned after a new virtual interface has been created
message CreateNetworkInterfaceResponse {
    string interface_name = 1;
}

// Request structure used to delete a virtual interface
message DeleteNetworkInterfaceRequest {
    string workload_id = 1;
}

// Request structure used to setup a new node's network
message SetupRequest {
    string ip_address = 1;
    string sub_network = 2;
}

// Response structure returned after a new node's network has been setup
message SetupResponse {
    string interface_name = 1;
}
```

### Services

```protobuf
service Network {
    // Create a new virtual inerface and add it to the node CNI
    rpc CreateNetworkInterface(CreateNetworkInterfaceRequest) returns (CreateNetworkInterfaceResponse) {}
    // Delete a virtual interface
    rpc DeleteNetworkInterface(DeleteNetworkInterfaceRequest) returns (Empty) {}
    // Create a new virtual network interface (CNI)
    rpc Setup(SetupRequest) returns (SetupResponse) {}
}
```
