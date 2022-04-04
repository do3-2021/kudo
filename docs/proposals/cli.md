# kudoctl documentation

kudoctl is the official cli client implementation for kudo.

## Global options

| short        | long                        | description                                                          |
| ------------ | --------------------------- | -------------------------------------------------------------------- |
| `-V`         | `--version`                 | output the version of the client TODO : define version format.       |
| `-h`         | `--help`                    | display the help text.                                               |
| `-c <path>`  | `--config <path>`           | specify the path to the config file.                                 |
|              | `--host <ip>`               | specify the ip of the control plane to connect to.                   |
| `-v <level>` | `--verbosity-level <level>` | Set the verbosity level of the execution, see **Log format** section |

## Exit codes

| Code | Category             | Description                                                                      |
| ---- | -------------------- | -------------------------------------------------------------------------------- |
| 0    | Success              | The command was successful.                                                      |
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

| Name     | Shorthand | Values               | Default        | Description                |
| -------- | --------- | -------------------- | -------------- | -------------------------- |
| `--help` | `-h`      |                      | false          | show help of the function. |
| `--fmt`  |           | json, human-readable | human-readable | set format.                |

**Examples** :

TODO

---

### get node \<id\>

Get detailed information about a node.

**Arguments** :

`id` : the id of the node

**Flags** :

| Name     | Shorthand | Values               | Default        | Description                |
| -------- | --------- | -------------------- | -------------- | -------------------------- |
| `--help` | `-h`      |                      | false          | show help of the function. |
| `--fmt`  |           | json, human-readable | human-readable | set format.                |

**Examples** :

TODO

---

### get workloads

---

### get workload \<id\>

---

### get instances

Get the list of instances and the name of the workload.

---

### get instance \<id\>

Get details about the instance.

**Arguments** :

`id` : the id of the instance

---

### delete workload \<id\>

Delete a workload definition.

**Arguments** :

`id` : the id of the workload

---

### delete instance \<id\>

Delete and stop an instance.

**Arguments** :

`id` : the id of the instance

---

### create workload

Create a workload definition

**Flags** :
`--file <file> | -f <file>` : add workload definition from file.

---

## instantiate workload \<id\>

Instantiate and start a workload

**Arguments** :

`id` : the id of the workload
