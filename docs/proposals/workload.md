# Workload schema

Example :

```json
{
    "kind" : "workload",
    "type": "container",
    "name": "test-workload",
    "uri": "nginx",
    "ressources": {
        "cpu": 1.0,
        "ram": 2048,
        "disk": 2,
    },
    "ports": [
        "8080:80"
    ],
    "environment": [
        "NODE_ENV=production"
    ]
}
```

## type

**type :** string

Type of the workload :

- `"container"`
- `"vm"`
- `"binary"`

## name

**type :** string

Name of the workload, should be unique in the controller’s database.

## uri

**type :** string

The name/uri of the container image , or an URI to the vm image or binary executable.

## ressources

**type :** object/struct

Definition of the maximum (container, binary) or allocated (vm) ressources.

### ressources.cpu

**type :** integer

CPU power, in number of milliCPU allocated.

### ressources.ram

**type :** integer

Memory amount, unit is MB.

### ressources.disk

**type :** integer

Disk size, unit is GB.

## ports

**type :** Array\<string\>

List of port mapping with the format `"external:internal"`.

## environment

**type :** Array\<string\>

List of environment variables to set before the execution of the workload, format is `"KEY=VALUE"`
