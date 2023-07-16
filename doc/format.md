# Rsync Log Analyzer - Format

## BNF

使用`BNF`格式的上下文无关文法简要描述 log 文件的格式，是

```bnf
file    = { line '\n' }
        ;
line    = date time '[' number ']' string
        | date time '[' number ']' 'cd+++++++++' directory
        | date time '[' number ']' '>f+++++++++' filepath
        ;
date    = number '/' number '/' number
        ;
time    = number ':' number ':' number
        ;
```

- terminators:

  - `number` : a integral literal (consists of `'0-9'`).

  - `string` : a string litearl (ends with `'\n'` or `EOF`).

  - `directory` : a string that describes directory path (ends with `'/'`).

  - `filepath` : a string that describes file path.

- non-terminators

  - `file` : content of the log file.

  - `line` : single line of the file, without `'\n'`.

  - `date` : for example `'2023/07/04'`.

  - `time` : for example `'03:44:47'`.

## Examples

1. 该行日志描述一个文件，特征是字符串`">f+++++++++"`。

```text
'2023/07/04 03:50:55 [14417] >f+++++++++ projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/signals2/detail/auto_buffer.hpp'
 ^~~~~~~~~~ ^~~~~~~~  ^~~~~              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 date       time      number             filepath
```

2. 该行日志描述一个文件夹，特征是字符串`"cd+++++++++"`。

```text
'2023/07/04 03:50:55 [14417] cd+++++++++ projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/serialization/'
 ^~~~~~~~~~ ^~~~~~~~  ^~~~~              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 date       time      number             directory
```

2. 程序不需要该类日志信息。

```text
'2023/07/04 03:40:27 [14417] total: matches=0  hash_hits=0  false_alarms=0 date=212534101301'
 ^~~~~~~~~~ ^~~~~~~~  ^~~~~  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 date       time      number string
```
