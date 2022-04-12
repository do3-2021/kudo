# Resource schema

A schema that can be sent to the controller in order to create a ressource.

The schema is sent as JSON to the controller. When passed as a file to the client the resource is using the YAML format with an added top level property `api_version` setting which specifies the version of the controller api to use.

Example :

```json
{
    "kind": "test",
    "name": "test",
    <kind-specific values ...>
}
```

## name

**type :** string  

Name of the ressource.

## kind

**type :** string

Type of the ressource :

- `workload` : see [workload.md](./workload.md).
- `user` : will be used for the creation of an user, the definition is not expected in v0.
