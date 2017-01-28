This crate helps to read contents of a file with name taken from environment variable.

[![Build Status](https://travis-ci.org/serejkus/env_file.svg?branch=master)](https://travis-ci.org/serejkus/env_file)

# Reading files with name taken from environment variable

Some applications have sensitive data, like API keys, which is unsafe to keep in VCS. One
possible solution is keeping secret data in files, taking their names from environment
variables. Say, you have to use cloud provider API key to control your PaaS system and you need
a key for external data API. You can then run your application like:

```
$ CLOUD_API_KEY=/etc/secrets/cloud_api.key DATA_API_KEY=/etc/secrets/data_api.key your_app
```

The same pattern simplifies testing (with test keys and not production ones). It is extendable
too: you may have a default location for production use with ability to override it via
environment variables.

# Example

```(rust)
extern crate env_file;

let api_key = read("CLOUD_API_KEY").unwrap_or("default_key".to_string());
```
