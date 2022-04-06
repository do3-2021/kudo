# kudoctl documentation

kudoctl is the official cli client implementation for kudo.

## Global flags

| Name                        | Shorthand    |            | Default        | Description                                                          |
| --------------------------- | ------------ | ---------- | -------------- | -------------------------------------------------------------------- |
| `--version`                 | `-V`         | bool       | false          | output the version of the client TODO : define version format.       |
| `--help`                    | `-h`         | bool       | false          | display the help text.                                               |
| `--config <path>`           | `-c <path>`  | path       | ~/.kudo/config | specify the path to the config file.                                 |
| `--host <ip>`               |              | string     | `''`           | specify the ip of the control plane to connect to.                   |
| `--verbosity-level <level>` | `-v <level>` | int 0 to 3 | 2              | Set the verbosity level of the execution, see **Log format** section |

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

| Name             | Shorthand | Values                                          | Default | Description                                                                                                                                                      |
| ---------------- | --------- | ----------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--help`         | `-h`      |                                                 | false   | show help of the function.                                                                                                                                       |
| `-format`        | `-fmt`    | `json`, `default`, `xml`, `delimit <character>` | default | Specifies the format of the output.                                                                                                                              |
| `-verbose`       | `-v`      |                                                 | false   | Specifies whether to enable verbose mode. Use the default value of off to disable verbose mode. This option is the default value. Use on to enable verbose mode. |
| `-page`          | `-p`      |                                                 | false   | Specifies whether to display one page of text at a time or all text at one time.                                                                                 |
| `-rows <number>` | `-r`      |                                                 | 24      | Specifies the number of rows per page to display when the **-p** parameter is on. You can specify a value in the range 1 - 100.                                  |
| `-header`        | `-hdr`    |                                                 | true    | Specifies whether to display the table header. Use the default value of on to display the table header. Use off to hide the table header.                        |

**Examples** :

TODO

---

### get node \<id\>

Get detailed information about a node.

**Arguments** :

`id` : the id of the node

**Flags** :

| Name      | Shorthand | Values                                          | Default | Description                         |
| --------- | --------- | ----------------------------------------------- | ------- | ----------------------------------- |
| `--help`  | `-h`      |                                                 | false   | show help of the function.          |
| `-format` | `-fmt`    | `json`, `default`, `xml`, `delimit <character>` | default | Specifies the format of the output. |

**Examples** :

TODO

---

### get workloads

Get a list of the workloads...

<!-- TODO: more description -->

**Flags** :

| Name             | Shorthand | Values                                          | Default | Description                                                                                                                                                      |
| ---------------- | --------- | ----------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--help`         | `-h`      |                                                 | false   | show help of the function.                                                                                                                                       |
| `-format`        | `-fmt`    | `json`, `default`, `xml`, `delimit <character>` | default | Specifies the format of the output.                                                                                                                              |
| `-verbose`       | `-v`      |                                                 | false   | Specifies whether to enable verbose mode. Use the default value of off to disable verbose mode. This option is the default value. Use on to enable verbose mode. |
| `-page`          | `-p`      |                                                 | false   | Specifies whether to display one page of text at a time or all text at one time.                                                                                 |
| `-rows <number>` | `-r`      |                                                 | 24      | Specifies the number of rows per page to display when the **-p** parameter is on. You can specify a value in the range 1 - 100.                                  |
| `-header`        | `-hdr`    |                                                 | true    | Specifies whether to display the table header. Use the default value of on to display the table header. Use off to hide the table header.                        |

---

### get workload \<id\>

This function returns the definition of a workload with the specified `id`.

**Arguments :**  

`id` : the id of the instance.

**Flags :**  

| Name     | Shorthand | Values              | Default | Description                                                            |
| -------- | --------- | ------------------- | ------- | ---------------------------------------------------------------------- |
| --format |           | `json`,`yaml`,`yml` | `yaml`  | The output format of the workload definition,  yml is the same as yaml |
---

### get instances

Get the list of instances and the name of the workload.

**Flags** :

| Name             | Shorthand | Values                                          | Default | Description                                                                                                                                                      |
| ---------------- | --------- | ----------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--help`         | `-h`      |                                                 | false   | show help of the function.                                                                                                                                       |
| `-format`        | `-fmt`    | `json`, `default`, `xml`, `delimit <character>` | default | Specifies the format of the output.                                                                                                                              |
| `-verbose`       | `-v`      |                                                 | false   | Specifies whether to enable verbose mode. Use the default value of off to disable verbose mode. This option is the default value. Use on to enable verbose mode. |
| `-page`          | `-p`      |                                                 | false   | Specifies whether to display one page of text at a time or all text at one time.                                                                                 |
| `-rows <number>` | `-r`      |                                                 | 24      | Specifies the number of rows per page to display when the **-p** parameter is on. You can specify a value in the range 1 - 100.                                  |
| `-header`        | `-hdr`    |                                                 | true    | Specifies whether to display the table header. Use the default value of on to display the table header. Use off to hide the table header.                        |

---

### get instance \<id\>

Get details about the instance.

**Arguments** :

`id` : the id of the instance

---

### delete workload \<id\>

Delete a workload definition.On success the command outputs no information.

**Arguments** :

`id` : the id of the workload

---

### delete instance \<id\>

Delete and stop an instance. On success the command outputs no information.

**Arguments** :

`id` : the id of the instance

---

### create workload

Create a workload definition

**Flags** :

| Name    | Shorthand | Values | Default | Description                        |
| ------- | --------- | ------ | ------- | ---------------------------------- |
| --files | -f        | Path   | `''`    | add workload definition from file. |

**Examples :**

- Add workload from file

  ```sh
  kudoctl create workload -f workload.yml
  ```

### apply

This command takes the same arguments as `create workload`, creates a workload, then instanciate it.

---

## instantiate workload \<id\>

Instantiate and start a workload

**Arguments** :

`id` : the id of the workload
