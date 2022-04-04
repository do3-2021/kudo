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
| 0    | Sucess               | The command was successful.                                                      |
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

___
### get nodes

Get a list of the nodes registered to the control plane.

**Parameters** :  

`--help | -h` : show help of the function

**Examples** :

TODO

___

### get node \<id\>

Get detailed informations about a node.

**Arguments** :  

`id` : the id of the node

**Parameters** :  

`--help | -h` : show help of the function.  
`--fmt` : set format TODO: add formats.  

**Examples** :  

TODO
____

### get workloads

___

### get workload \<id\>

___

### get instances

___

### get instance \<id\>

___

### delete workload \<id\>

___

### delete instance \<id\>

___

### create workload

___

## instantiate workload \<id\>
