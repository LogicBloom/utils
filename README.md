# chatterverse_utils

A collection of reusable configs in chatterverse core projects

The utils crate basically handles common set up code that is done in multiple projects to keep things DRY, the code is generic enough to be used in any project hence why it's open source.

The following functionalities are handled by the crate:

- configuration set up using the `config` crate. A `get_configurations::<T: Deserialize>` function is exposed. Also, `yaml` file configs are preferred for now
- telemetry set up by initialization of a tracing subscriber via a `initialize_tracing_subscriber` function

## usage:

```toml
[dependencies]
chatterverse_utils = {git = "https://github.com/chatterverse-ai/utils.git}
```