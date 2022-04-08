# kudoctl documentation

kudoctl is the official cli client implementation for kudo.

## Global flags

| Name              | Shorthand | Values     | Default        | Description                                                          |
| ----------------- | --------- | ---------- | -------------- | -------------------------------------------------------------------- |
| --version         | -V        | bool       | false          | output the version of the client TODO : define version format.       |
| --help            | -h        | bool       | false          | display the help text.                                               |
| --config          | -c        | path       | ~/.kudo/config | specify the path to the config file.                                 |
| --host            |           | string     | `''`           | specify the ip of the control plane to connect to.                   |
| --verbosity-level | -v        | int 0 to 3 | 2              | Set the verbosity level of the execution, see **Log format** section |

## Exit codes

| Code | Category             | Description                                                                      |
| ---- | -------------------- | -------------------------------------------------------------------------------- |
| 0    | Success              | The command was successful.                                                      |
| 1    | Unexpected error     | This is an unhandled error and it should never happen in normal usage            |
| 2    | Syntax error         | The syntax of the command was not correct.                                       |
| 3    | Connection error     | A connectivity error or protocol error occurred.                                 |
| 4    | Server error         | An error occurred during a call to the controller.                               |
| 5    | Authentication error | An error was detected during authentication checking.                            |
| 6    | Application error    | An error occurred during processing that is performed by the client application. |

## Log format

| Severity    | level | format              |
| ----------- | ----- | ------------------- |
| Debug       | 3     | `Debug : MESSAGE`   |
| Information | 2     | `MESSAGE`           |
| Warning     | 1     | `Warning : MESSAGE` |
| Error       | 0     | `Error : MESSAGE`   |

## Commands

Command format : `kudoctl [global options] <command> [command options]`.

---

### get nodes

Get a list of the nodes registered to the control plane.

**Flags** :

| Name      | Shorthand | Values                                                  | Default     | Description                                                                                                                                                      |
| --------- | --------- | ------------------------------------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --help    | -h        |                                                         | false       | show help of the function.                                                                                                                                       |
| --format  | -F        | `"json"`, `"default"`, `"xml"`, `"delimit <character>"` | `"default"` | Specifies the format of the output.                                                                                                                              |
| --verbose | -v        |                                                         | false       | Specifies whether to enable verbose mode. Use the default value of off to disable verbose mode. This option is the default value. Use on to enable verbose mode. |
| --page    | -p        |                                                         | false       | Specifies whether to display one page of text at a time or all text at one time.                                                                                 |
| --rows    | -r        | integer                                                 | 24          | Specifies the number of rows per page to display when the **-p** parameter is on. You can specify a value in the range 1 - 100.                                  |
| --header  | -h        |                                                         | true        | Specifies whether to display the table header. Use the default value of on to display the table header. Use off to hide the table header.                        |

**Examples** :

TODO

---

### get node \<id\>

Get detailed information about a node.

**Arguments** :

`id` : the id of the node

**Flags** :

| Name     | Shorthand | Values                                                  | Default     | Description                         |
| -------- | --------- | ------------------------------------------------------- | ----------- | ----------------------------------- |
| --help   | -h        |                                                         | false       | show help of the function.          |
| --format | -F        | `"json"`, `"default"`, `"xml"`, `"delimit <character>"` | `"default"` | Specifies the format of the output. |

**Examples** :

TODO

---

### get resources

Get a list of the resources...

<!-- TODO: more description -->

**Flags** :

| Name      | Shorthand | Values                                                  | Default     | Description                                                                                                                                                      |
| --------- | --------- | ------------------------------------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --help    | -h        |                                                         | false       | show help of the function.                                                                                                                                       |
| --format  | -F        | `"json"`, `"default"`, `"xml"`, `"delimit <character>"` | `"default"` | Specifies the format of the output.                                                                                                                              |
| --verbose | -v        |                                                         | false       | Specifies whether to enable verbose mode. Use the default value of off to disable verbose mode. This option is the default value. Use on to enable verbose mode. |
| --page    | -p        |                                                         | false       | Specifies whether to display one page of text at a time or all text at one time.                                                                                 |
| --rows    | -r        | integer                                                 | 24          | Specifies the number of rows per page to display when the **-p** parameter is on. You can specify a value in the range 1 - 100.                                  |
| --header  | -h        |                                                         | true        | Specifies whether to display the table header. Use the default value of on to display the table header. Use off to hide the table header.                        |

---

### get resource \<id\>

This function returns the definition of a resource with the specified `id`.

**Arguments :**  

`id` : the id of the instance.

**Flags :**  

| Name      | Shorthand | Values                    | Default  | Description                                                            |
| --------- | --------- | ------------------------- | -------- | ---------------------------------------------------------------------- |
| ---format |           | `"json"`,`"yaml"`,`"xml"` | `"yaml"` | The output format of the resource definition,  yml is the same as yaml |
---

### get instances

Get the list of instances and the name of the resource.

**Flags** :

| Name      | Shorthand | Values                                                  | Default     | Description                                                                                                                                                      |
| --------- | --------- | ------------------------------------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --help    | -h        |                                                         | false       | show help of the function.                                                                                                                                       |
| --format  | -F        | `"json"`, `"default"`, `"xml"`, `"delimit <character>"` | `"default"` | Specifies the format of the output.                                                                                                                              |
| --verbose | -v        |                                                         | false       | Specifies whether to enable verbose mode. Use the default value of off to disable verbose mode. This option is the default value. Use on to enable verbose mode. |
| --page    | -p        |                                                         | false       | Specifies whether to display one page of text at a time or all text at one time.                                                                                 |
| --rows    | -r        | integer                                                 | 24          | Specifies the number of rows per page to display when the **-p** parameter is on. You can specify a value in the range 1 - 100.                                  |
| --header  | -h        |                                                         | true        | Specifies whether to display the table header. Use the default value of on to display the table header. Use off to hide the table header.                        |

---

### get instance \<id\>

Get details about the instance.

**Arguments** :

`id` : the id of the instance

---

### delete resource \<id\>

Delete a resource definition and all the instances of this resource. On success the command outputs no information.

**Arguments** :

`id` : the id of the resource

---

### delete instance \<id\>

Delete and stop an instance. On success the command outputs no information.

**Arguments** :

`id` : the id of the instance.

---

### create \<kind\>

Create a resource definition. By default if a resource with the same name exists, the resource will be updated, add the `--no-update` flag if you don’t want this behavior.

**Arguments :**

`kind` : the kind of the ressource, possible values :

- workload

**Flags** :

| Name        | Shorthand | Values | Default | Description                                     |
| ----------- | --------- | ------ | ------- | ----------------------------------------------- |
| --file      | -f        | Path   | `""`    | add resource definition from file.              |
| --no-update |           | bool   | false   | If the resource already exists, don’t update it |
| --name      |           | string | `""`    | set the name of the resource                    |

**Kind specific flags :**

*workload :*

| Name             | Shorthand | Values   | Default       | Description                                                                           |
| ---------------- | --------- | -------- | ------------- | ------------------------------------------------------------------------------------- |
| --type           |           | string   | `"container"` | workload type                                                                         |
| --uri            |           | string   | `""`          | the uri                                                                               |
| --resources-cpu  |           | integer  | 1             | the cpu amount                                                                        |
| --resources-ram  |           | integer  | 50            | the ram amount      (MB)                                                              |
| --resources-disk |           | integer  | 1             | the disk size (GB)                                                                    |
| --port           | -p        | []string | []            | ports binding list, use multiple times to add multiple elements to the array          |
| --environment    |           | []string | []            | environment variables list, use multiple times to add multiple elements  to the array |

**Examples :**

- Add resource from file

  ```sh
  kudoctl create resource -f workload.yml
  ```

### apply

This command takes the same arguments as `create <kind>` except the kind is defined by the `--kind` flag , creates a resource, then instanciate it.

**Flags :**

| Name   | Shorthand | Values | Default      | Description    |
| ------ | --------- | ------ | ------------ | -------------- |
| --kind |           | string | `"workload"` | ressource kind |

---

## instantiate \<ressource-id\>

Instantiate and start a resource

**Arguments** :

`ressource-id` : the id of the resource
